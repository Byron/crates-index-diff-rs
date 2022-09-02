use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use git_repository as git;
use git_repository::diff::tree::visit::Action;
use similar::ChangeTag;
use std::collections::BTreeSet;

pub(crate) struct Delegate<'repo> {
    changes: Vec<Change>,
    delete_version_ids: BTreeSet<u64>,
    file_name: git::bstr::BString,
    err: Option<Error>,
    repo: &'repo git::Repository,
}

impl<'repo> Delegate<'repo> {
    pub fn from_repo(repo: &'repo git::Repository) -> Self {
        Delegate {
            changes: Vec::new(),
            delete_version_ids: BTreeSet::new(),
            err: None,
            file_name: Default::default(),
            repo,
        }
    }
    fn handle(&mut self, change: git::diff::tree::visit::Change) -> Result<(), Error> {
        use git::diff::tree::visit::Change::*;
        use git::objs::tree::EntryMode::*;
        fn entry_data(
            repo: &git::Repository,
            entry: git::objs::tree::EntryMode,
            oid: git::hash::ObjectId,
        ) -> Result<Option<git::Object<'_>>, Error> {
            matches!(entry, Blob | BlobExecutable)
                .then(|| repo.find_object(oid))
                .transpose()
                .map_err(Into::into)
        }
        use git::bstr::ByteSlice;
        match change {
            Addition { entry_mode, oid } => {
                if let Some(obj) = entry_data(self.repo, entry_mode, oid)? {
                    for line in (&obj.data).lines() {
                        let version = version_from_json_line(line)?;
                        self.changes.push(if version.yanked {
                            Change::Yanked(version)
                        } else {
                            Change::Added(version)
                        });
                    }
                }
            }
            Deletion { .. } => {
                self.changes.push(Change::Deleted {
                    name: self.file_name.to_string(),
                });
            }
            Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
            } => {
                let pair = entry_data(self.repo, previous_entry_mode, previous_oid)?
                    .zip(entry_data(self.repo, entry_mode, oid)?);
                if let Some((old, new)) = pair {
                    let diff = similar::TextDiffConfig::default()
                        .algorithm(similar::Algorithm::Myers)
                        .diff_lines(old.data.as_slice(), new.data.as_slice());
                    for change in diff.iter_all_changes() {
                        match change.tag() {
                            ChangeTag::Delete | ChangeTag::Insert => {
                                let version = version_from_json_line(change.value())?;
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
        Ok(())
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

impl git::diff::tree::Visit for Delegate<'_> {
    fn pop_front_tracked_path_and_set_current(&mut self) {}
    fn push_back_tracked_path_component(&mut self, _component: &git::bstr::BStr) {}
    fn push_path_component(&mut self, component: &git::bstr::BStr) {
        use git::bstr::ByteVec;
        self.file_name.clear();
        self.file_name.push_str(component);
    }
    fn pop_path_component(&mut self) {}

    fn visit(&mut self, change: git::diff::tree::visit::Change) -> Action {
        match self.handle(change) {
            Ok(()) => Action::Continue,
            Err(err) => {
                self.err = err.into();
                Action::Cancel
            }
        }
    }
}

fn version_from_json_line(line: &[u8]) -> Result<CrateVersion, Error> {
    serde_json::from_slice(line).map_err(|err| Error::VersionDecode {
        source: err,
        line: line.into(),
    })
}
