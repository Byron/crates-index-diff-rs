use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use ahash::AHashSet;
use bstr::BStr;
use git_repository as git;
use std::hash::Hash;

#[repr(transparent)]
struct ChecksumWithVersion(CrateVersion);

impl AsRef<ChecksumWithVersion> for CrateVersion {
    fn as_ref(&self) -> &ChecksumWithVersion {
        // Safety: this is safe because ChecksumWithVersion is just a repr[transparent] wrapper around CrateVersion
        unsafe { &*(self as *const CrateVersion as *const ChecksumWithVersion) }
    }
}
impl Hash for ChecksumWithVersion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.checksum.hash(state)
    }
}

impl PartialEq for ChecksumWithVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0.checksum == other.0.checksum
    }
}
impl Eq for ChecksumWithVersion {}

#[derive(Default)]
pub(crate) struct Delegate {
    changes: Vec<Change>,
    err: Option<Error>,
    temporary_line_map: AHashSet<&'static [u8]>,
    temporary_version_map: AHashSet<ChecksumWithVersion>,
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
                    let location = change.location;
                    for line in diff.old.data.lines() {
                        // Safety: We transform an &'_ [u8] to and &'static [u8] here
                        // this is safe because we always drain the hashmap at the end of the function
                        // the reason the HashMap has a static is that we want to reuse
                        // the allocation for modifications
                        self.temporary_line_map
                            .insert(unsafe { &*(line as *const [u8]) });
                    }

                    for line in diff.new.data.lines() {
                        // first quickly check if the exact same line is already present in this file in that case we don't need to do anything else
                        if self.temporary_line_map.remove(line) {
                            continue;
                        }
                        let new_version = version_from_json_line(line, location)?;
                        self.temporary_version_map
                            .insert(ChecksumWithVersion(new_version));
                    }

                    let mut deleted = Vec::new();
                    for line in self.temporary_line_map.drain() {
                        let old_version = version_from_json_line(line, location)?;
                        let new_version = self.temporary_version_map.take(old_version.as_ref());
                        match new_version {
                            Some(ChecksumWithVersion(new_version)) => {
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
                    for ChecksumWithVersion(version) in self.temporary_version_map.drain() {
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
