use crates_index_diff::Index;

mod old;
mod changes_from_objects {
    use crate::index::index_ro;
    use crates_index_diff::{Change, CrateVersion, Index};
    use git_repository as git;
    use git_repository::prelude::ObjectIdExt;

    #[test]
    fn addition() -> crate::Result {
        let _index = index_ro()?;
        let changes = changes(&_index, "initial commit")?;
        assert_eq!(changes.len(), 3228);
        assert!(matches!(
            changes.first().expect("present"),
            Change::Added(CrateVersion {name, ..}) if name == "gi-get-artifact"
        ));
        assert!(matches!(
            changes.last().expect("present"),
            Change::Added(CrateVersion {name, ..}) if name == "gizmo"
        ));
        Ok(())
    }

    fn changes(index: &Index, commit_message: &str) -> crate::Result<Vec<Change>> {
        let repo = git::open(index.repository().path())?;
        let commit = repo
            .rev_parse(format!(":/{commit_message}").as_str())?
            .single()
            .unwrap();
        let ancestor_tree = commit
            .object()?
            .into_commit()
            .parent_ids()
            .next()
            .and_then(|parent| parent.object().ok()?.into_commit().tree_id().ok())
            .unwrap_or(git::hash::ObjectId::empty_tree(repo.object_hash()).attach(&repo));
        Ok(index.changes_from_objects(
            &index
                .repository()
                .find_object(object_id_to_oid(ancestor_tree), None)
                .expect("ancestor tree is available"),
            &index
                .repository()
                .find_object(object_id_to_oid(commit), None)
                .expect("first object exists"),
        )?)
    }

    fn object_id_to_oid(oid: impl Into<git::ObjectId>) -> git2::Oid {
        git2::Oid::from_bytes(oid.into().as_bytes()).expect("valid")
    }
}

fn index_ro() -> crate::Result<Index> {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )?;
    Ok(Index::from_path_or_cloned(dir.join("base"))?)
}
