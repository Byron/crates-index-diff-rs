use crate::index::index_ro;
use crates_index_diff::{Change, CrateVersion, Index};
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

#[test]
fn addition() -> crate::Result {
    let changes = changes(&index_ro()?, ":/initial commit")?;
    assert_eq!(changes.len(), 3228);
    assert!(matches!(
        changes
            .first()
            .and_then(|c| c.added().map(|v| v.name.as_str())),
        Some("gi-get-artifact")
    ));
    assert!(matches!(
        changes.last().expect("present"),
        Change::Added(CrateVersion {name, ..}) if name == "gizmo"
    ));
    Ok(())
}

#[test]
fn deletion() -> crate::Result {
    let changes = changes(&index_ro()?, "@~326")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.first().and_then(|c| c.deleted()), Some("girl"));
    Ok(())
}

#[test]
fn new_version() -> crate::Result {
    let changes = changes(&index_ro()?, ":/Updating crate `git-repository#0.22.1`")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.added().map(|v| v.name.as_str())),
        Some("git-repository")
    );
    Ok(())
}

#[test]
fn yanked() -> crate::Result {
    let changes = changes(&index_ro()?, ":/Yanking crate `github_release_rs#0.1.0`")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.yanked().map(|v| v.name.as_str())),
        Some("github_release_rs")
    );
    Ok(())
}

#[test]
fn normalization() -> crate::Result {
    let changes = changes(&index_ro()?, ":/normalize")?;
    assert_eq!(
        changes.len(),
        2356, // should be 0
        "normalization changes the representation, but the data itself stays the same, BUT we can't do it yet"
    );
    Ok(())
}

fn changes(index: &Index, revspec: &str) -> crate::Result<Vec<Change>> {
    let repo = git::open(index.repository().path())?;
    let commit = repo.rev_parse(revspec)?.single().unwrap();
    let ancestor_tree = commit
        .object()?
        .into_commit()
        .parent_ids()
        .next()
        .and_then(|parent| parent.object().ok()?.into_commit().tree_id().ok())
        .unwrap_or_else(|| git::hash::ObjectId::empty_tree(repo.object_hash()).attach(&repo));
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
