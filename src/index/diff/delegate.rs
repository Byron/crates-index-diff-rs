use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use ahash::{AHashSet, RandomState};
use bstr::BStr;
use hashbrown::HashTable;
use std::hash::Hasher;
use std::ops::Deref;

#[derive(Default)]
pub(crate) struct Delegate {
    changes: Vec<Change>,
    /// All changes that happen within a file, along the line-number it happens in .
    per_file_changes: Vec<(usize, Change)>,
    err: Option<Error>,
}

impl Delegate {
    pub fn handle(
        &mut self,
        change: gix::object::tree::diff::Change<'_, '_, '_>,
    ) -> Result<gix::object::tree::diff::Action, Error> {
        use gix::bstr::ByteSlice;
        use gix::object::tree::diff::Change::*;
        use gix::objs::tree::EntryKind::*;
        fn entry_data(
            entry: gix::objs::tree::EntryKind,
            id: gix::Id<'_>,
        ) -> Result<Option<gix::Object<'_>>, Error> {
            matches!(entry, Blob | BlobExecutable)
                .then(|| id.object())
                .transpose()
                .map_err(Into::into)
        }
        if change.location().contains(&b'.') {
            return Ok(Default::default());
        }

        match change {
            Rewrite { .. } => {
                unreachable!("BUG: this is disabled so shouldn't happen")
            }
            Addition {
                entry_mode,
                id,
                location,
                ..
            } => {
                if let Some(obj) = entry_data(entry_mode.kind(), id)? {
                    for line in obj.data.lines() {
                        let version = version_from_json_line(line, location)?;
                        let change = if version.yanked {
                            Change::AddedAndYanked(version)
                        } else {
                            Change::Added(version)
                        };
                        self.changes.push(change)
                    }
                }
            }
            Deletion {
                entry_mode,
                id,
                location,
                ..
            } => {
                if entry_mode.is_no_tree() {
                    let obj = id.object()?;
                    let mut deleted = Vec::with_capacity(obj.data.lines().count());
                    for line in obj.data.lines() {
                        deleted.push(version_from_json_line(line, location)?);
                    }
                    self.changes.push(Change::CrateDeleted {
                        name: location.to_string(),
                        versions: deleted,
                    });
                }
            }
            Modification {
                entry_mode,
                previous_id,
                id,
                location,
                ..
            } => {
                if entry_mode.is_blob() {
                    let old = previous_id.object()?.into_blob();
                    let new = id.object()?.into_blob();
                    let mut old_lines = AHashSet::with_capacity(1024);
                    let location = location;
                    for (number, line) in old.data.lines().enumerate() {
                        old_lines.insert(Line(number, line));
                    }

                    // A HashTable is used to represent a Checksum -> CrateVersion map
                    // because the checksum is already stored in the CrateVersion
                    // and we want to avoid storing the checksum twice for performance reasons
                    let mut new_versions = HashTable::with_capacity(old_lines.len().min(1024));
                    let hasher = RandomState::new();

                    for (number, line) in new.data.lines().enumerate() {
                        // first quickly check if the exact same line is already present in this file in that case we don't need to do anything else
                        if old_lines.remove(&Line(number, line)) {
                            continue;
                        }
                        // no need to check if the checksum already exists in the hashmap
                        // as each checksum appears only once
                        let new_version = version_from_json_line(line, location)?;
                        new_versions.insert_unique(
                            hasher.hash_one(new_version.checksum),
                            (number, new_version),
                            |rehashed| hasher.hash_one(rehashed.1.checksum),
                        );
                    }

                    for line in old_lines.drain() {
                        let old_version = version_from_json_line(&line, location)?;
                        let new_version: Option<(usize, CrateVersion)> = new_versions
                            .find_entry(hasher.hash_one(old_version.checksum), |version| {
                                version.1.checksum == old_version.checksum
                            })
                            .map(|entry| entry.remove().0)
                            .ok();
                        match new_version {
                            Some((_, new_version)) => {
                                let change = match (old_version.yanked, new_version.yanked) {
                                    (true, false) => Change::Unyanked(new_version),
                                    (false, true) => Change::Yanked(new_version),
                                    _ => continue,
                                };
                                self.per_file_changes.push((line.0, change))
                            }
                            None => self
                                .per_file_changes
                                .push((line.0, Change::VersionDeleted(old_version))),
                        }
                    }
                    for (number, version) in new_versions.drain() {
                        let change = if version.yanked {
                            Change::AddedAndYanked(version)
                        } else {
                            Change::Added(version)
                        };
                        self.per_file_changes.push((number, change));
                    }
                    self.per_file_changes.sort_by_key(|t| t.0);
                    self.changes
                        .extend(self.per_file_changes.drain(..).map(|t| t.1));
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

/// A line that assumes there never are equal lines within a file which
/// is the case due to the checksum.
struct Line<'a>(usize, &'a [u8]);

impl std::hash::Hash for Line<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state)
    }
}

impl PartialEq<Self> for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(other.1)
    }
}

impl Eq for Line<'_> {}

impl<'a> Deref for Line<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.1
    }
}

fn version_from_json_line(line: &[u8], file_name: &BStr) -> Result<CrateVersion, Error> {
    serde_json::from_slice(line).map_err(|err| Error::VersionDecode {
        source: err,
        file_name: file_name.into(),
        line: line.into(),
    })
}
