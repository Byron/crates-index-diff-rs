use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use ahash::{AHashSet, RandomState};
use bstr::BStr;
use git_repository as git;
use hashbrown::raw::RawTable;

#[derive(Default)]
pub(crate) struct Delegate {
    changes: Vec<Change>,
    err: Option<Error>,
}

impl Delegate {
    pub fn handle(
        &mut self,
        change: git::object::tree::diff::Change<'_, '_, '_>,
    ) -> Result<git::object::tree::diff::Action, Error> {
        use git::bstr::ByteSlice;
        use git::object::tree::diff::change::Event::*;
        use git::objs::tree::EntryMode::*;
        fn entry_data(
            entry: git::objs::tree::EntryMode,
            id: git::Id<'_>,
        ) -> Result<Option<git::Object<'_>>, Error> {
            matches!(entry, Blob | BlobExecutable)
                .then(|| id.object())
                .transpose()
                .map_err(Into::into)
        }
        if change.location.contains(&b'.') {
            return Ok(Default::default());
        }

        match change.event {
            Addition { entry_mode, id } => {
                if let Some(obj) = entry_data(entry_mode, id)? {
                    for line in obj.data.lines() {
                        let version = version_from_json_line(line, change.location)?;
                        let change = if version.yanked {
                            Change::AddedAndYanked(version)
                        } else {
                            Change::Added(version)
                        };
                        self.changes.push(change)
                    }
                }
            }
            Deletion { entry_mode, id, .. } => {
                if entry_mode.is_no_tree() {
                    let obj = id.object()?;
                    let mut deleted = Vec::with_capacity(obj.data.lines().count());
                    for line in obj.data.lines() {
                        deleted.push(version_from_json_line(line, change.location)?);
                    }
                    self.changes.push(Change::Deleted {
                        name: change.location.to_string(),
                        versions: deleted,
                    });
                }
            }
            Modification { .. } => {
                if let Some(diff) = change.event.diff().transpose()? {
                    let mut old_lines = AHashSet::with_capacity(1024);
                    let location = change.location;
                    for line in diff.old.data.lines() {
                        // Safety: We transform an &'_ [u8] to and &'static [u8] here
                        // this is safe because we always drain the hashmap at the end of the function
                        // the reason the HashMap has a static is that we want to reuse
                        // the allocation for modifications
                        old_lines.insert(line);
                    }

                    // A RawTable is used to represent a Checksum -> CrateVersion map
                    // because the checksum is already stored in the CrateVersion
                    // and we want to avoid storing the checksum twice for performance reasons
                    let mut new_versions = RawTable::with_capacity(old_lines.len().min(1024));
                    let hasher = RandomState::new();

                    for line in diff.new.data.lines() {
                        // first quickly check if the exact same line is already present in this file in that case we don't need to do anything else
                        if old_lines.remove(line) {
                            continue;
                        }
                        // no need to check if the checksum already exists in the hashmap
                        // as each checksum appear only once
                        let new_version = version_from_json_line(line, location)?;
                        new_versions.insert(
                            hasher.hash_one(new_version.checksum),
                            new_version,
                            |rehashed| hasher.hash_one(rehashed.checksum),
                        );
                    }

                    let mut deleted = Vec::new();
                    for line in old_lines.drain() {
                        let old_version = version_from_json_line(line, location)?;
                        let new_version = new_versions
                            .remove_entry(hasher.hash_one(old_version.checksum), |version| {
                                version.checksum == old_version.checksum
                            });
                        match new_version {
                            Some(new_version) => {
                                let change = match (old_version.yanked, new_version.yanked) {
                                    (true, false) => Change::Unyanked(new_version),
                                    (false, true) => Change::Yanked(new_version),
                                    _ => continue,
                                };
                                self.changes.push(change)
                            }
                            None => deleted.push(old_version),
                        }
                    }
                    if !deleted.is_empty() {
                        self.changes.push(Change::Deleted {
                            name: deleted[0].name.to_string(),
                            versions: deleted,
                        })
                    }
                    for version in new_versions.drain() {
                        let change = if version.yanked {
                            Change::AddedAndYanked(version)
                        } else {
                            Change::Added(version)
                        };
                        self.changes.push(change);
                    }
                }
            }
        }
        Ok(Default::default())
    }

    pub fn into_result(self) -> Result<Vec<Change>, Error> {
        match self.err {
            Some(err) => Err(err),
            None => Ok(self.changes),
        }
    }
}

fn version_from_json_line(line: &[u8], file_name: &BStr) -> Result<CrateVersion, Error> {
    serde_json::from_slice(line).map_err(|err| Error::VersionDecode {
        source: err,
        file_name: file_name.into(),
        line: line.into(),
    })
}
