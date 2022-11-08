use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use bstr::BStr;
use git_repository as git;
use std::collections::BTreeSet;
use std::ops::Range;

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
        let mut line_changes = Vec::new();
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
            Modification { .. } => {
                if let Some(diff) = change.event.diff().transpose()? {
                    let location = change.location;

                    enum Op {
                        Delete,
                        Add,
                    }
                    type SinkOutput = Vec<Result<(CrateVersion, Op), Error>>;
                    struct Sink<'a, 'b, 'c> {
                        out: &'a mut SinkOutput,
                        input: &'b git::diff::text::imara::intern::InternedInput<&'b [u8]>,
                        location: &'c BStr,
                    }

                    impl<'a, 'b, 'c> git::diff::text::imara::Sink for Sink<'a, 'b, 'c> {
                        type Out = &'a mut SinkOutput;

                        fn process_change(&mut self, before: Range<u32>, after: Range<u32>) {
                            let mut line_before = self.input.before
                                [before.start as usize..before.end as usize]
                                .iter()
                                .map(|&line| self.input.interner[line]);
                            let mut line_after = self.input.after
                                [after.start as usize..after.end as usize]
                                .iter()
                                .map(|&line| self.input.interner[line]);
                            match (line_before.next(), line_after.next()) {
                                (Some(removed), None) => {
                                    self.out.push(
                                        version_from_json_line(removed.as_bstr(), self.location)
                                            .map(|v| (v, Op::Delete)),
                                    );
                                }
                                (None, Some(inserted)) => {
                                    self.out.push(
                                        version_from_json_line(inserted.as_bstr(), self.location)
                                            .map(|v| (v, Op::Add)),
                                    );
                                }
                                (Some(_), Some(_)) | (None, None) => {
                                    /* ignore modifications, shouldn't exist */
                                }
                            }
                        }

                        fn finish(self) -> Self::Out {
                            self.out
                        }
                    }

                    let sink =
                        |input: &git::diff::text::imara::intern::InternedInput<&[u8]>| Sink {
                            out: &mut line_changes,
                            input,
                            location,
                        };
                    for op in diff.lines(sink).drain(..) {
                        let (version, op) = op?;
                        match op {
                            Op::Add => {
                                self.changes.push(if version.yanked {
                                    Change::Yanked(version)
                                } else {
                                    Change::Added(version)
                                });
                            }
                            Op::Delete => {
                                self.delete_version_ids.insert(version.id());
                            }
                        };
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
