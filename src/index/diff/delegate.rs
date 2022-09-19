use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use bstr::BStr;
use git_repository as git;
use similar::ChangeTag;
use std::collections::BTreeSet;

#[derive(Default)]
pub(crate) struct Delegate {
    changes: Vec<Change>,
    delete_version_ids: BTreeSet<u64>,
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
                    for line in (&obj.data).lines() {
                        let version = version_from_json_line(line, change.location)?;
                        self.changes.push(if version.yanked {
                            Change::Yanked(version)
                        } else {
                            Change::Added(version)
                        });
                    }
                }
            }
            Deletion { entry_mode, .. } => {
                if entry_mode.is_no_tree() {
                    self.changes.push(Change::Deleted {
                        name: change.location.to_string(),
                    });
                }
            }
            Modification {
                previous_entry_mode,
                previous_id,
                entry_mode,
                id,
            } => {
                let pair =
                    entry_data(previous_entry_mode, previous_id)?.zip(entry_data(entry_mode, id)?);
                if let Some((old, new)) = pair {
                    let diff = similar::TextDiffConfig::default()
                        .algorithm(similar::Algorithm::Myers)
                        .diff_lines(old.data.as_slice(), new.data.as_slice());
                    let location = change.location;
                    for change in diff.iter_all_changes() {
                        match change.tag() {
                            ChangeTag::Delete | ChangeTag::Insert => {
                                let version = version_from_json_line(change.value(), location)?;
                                if change.tag() == ChangeTag::Insert {
                                    self.changes.push(if version.yanked {
                                        Change::Yanked(version)
                                    } else {
                                        Change::Added(version)
                                    });
                                } else {
                                    self.delete_version_ids.insert(version.id());
                                }
                            }
                            ChangeTag::Equal => {}
                        }
                    }
                }
            }
        }
        Ok(Default::default())
    }

    pub fn into_result(mut self) -> Result<Vec<Change>, Error> {
        match self.err {
            Some(err) => Err(err),
            None => {
                if !self.delete_version_ids.is_empty() {
                    let deleted_version_ids = &self.delete_version_ids;
                    self.changes.retain(|change| match change {
                        Change::Added(v) | Change::Yanked(v) => {
                            !deleted_version_ids.contains(&v.id())
                        }
                        Change::Deleted { .. } => true,
                    })
                }
                Ok(self.changes)
            }
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
