use crate::index::diff::Error;
use crate::{Change, CrateVersion};
use bstr::BStr;
use git_repository as git;
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Default)]
pub(crate) struct Delegate {
    map: BTreeMap<[u8; 32], CrateVersion>,
    deletions: Vec<Change>,
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
                        self.map.insert(version.checksum, version.into());
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
                    self.deletions.push(Change::Deleted {
                        name: change.location.to_string(),
                        versions: deleted,
                    });
                }
            }
            Modification { .. } => {
                if let Some(diff) = change.event.diff().transpose()? {
                    let location = change.location;

                    let input = diff.line_tokens();
                    let mut err = None;
                    git::diff::blob::diff(
                        diff.algo,
                        &input,
                        |before: Range<u32>, after: Range<u32>| {
                            if err.is_some() {
                                return;
                            }
                            let mut lines_before = input.before
                                [before.start as usize..before.end as usize]
                                .iter()
                                .map(|&line| input.interner[line].as_bstr())
                                .peekable();
                            let mut lines_after = input.after
                                [after.start as usize..after.end as usize]
                                .iter()
                                .map(|&line| input.interner[line].as_bstr())
                                .peekable();
                            'outer: loop {
                                match (lines_before.peek().is_some(), lines_after.peek().is_some())
                                {
                                    (true, false) => {
                                        for removed in lines_before {
                                            match version_from_json_line(removed, location) {
                                                Ok(version) => {
                                                    self.map.insert(version.checksum, version);
                                                }
                                                Err(e) => {
                                                    err = Some(e);
                                                    break;
                                                }
                                            }
                                        }
                                        break 'outer;
                                    }
                                    (false, true) => {
                                        for inserted in lines_after {
                                            match version_from_json_line(inserted, location) {
                                                Ok(version) => {
                                                    self.map
                                                        .insert(version.checksum, version.into());
                                                }
                                                Err(e) => {
                                                    err = Some(e);
                                                    break;
                                                }
                                            }
                                        }
                                        break 'outer;
                                    }
                                    (true, true) => {
                                        for (removed, inserted) in
                                            lines_before.by_ref().zip(lines_after.by_ref())
                                        {
                                            match version_from_json_line(inserted, location)
                                                .and_then(|inserted| {
                                                    version_from_json_line(removed, location)
                                                        .map(|removed| (removed, inserted))
                                                }) {
                                                Ok((removed_version, inserted_version)) => {
                                                    self.map.insert(
                                                        removed_version.checksum,
                                                        removed_version,
                                                    );
                                                    self.map.insert(
                                                        inserted_version.checksum,
                                                        inserted_version,
                                                    );
                                                }
                                                Err(e) => {
                                                    err = Some(e);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    (false, false) => break 'outer,
                                }
                            }
                        },
                    );
                    if let Some(err) = err {
                        return Err(err);
                    }
                }
            }
        }
        Ok(Default::default())
    }

    pub fn into_result(self) -> Result<Vec<Change>, Error> {
        match self.err {
            Some(err) => Err(err),
            None => {
                let mut changes = self.map.into_values().map(Into::into).collect::<Vec<_>>();
                changes.sort_by(|a: &Change, b: &Change| {
                    let a = a.version();
                    let b = b.version();
                    a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version))
                });
                changes.extend(self.deletions);
                Ok(changes)
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
