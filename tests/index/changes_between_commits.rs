use crate::index::index_ro;
use crates_index_diff::{Change, CrateVersion, Index};
use git_repository as git;

#[test]
fn addition() -> crate::Result {
    let changes = changes(index_ro()?, ":/initial commit")?;
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
    let changes = changes(index_ro()?, "@~327")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.first().and_then(|c| c.deleted()), Some("girl"));
    Ok(())
}

#[test]
fn new_version() -> crate::Result {
    let changes = changes(index_ro()?, ":/Updating crate `git-repository#0.22.1`")?;
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
    let changes = changes(index_ro()?, ":/Yanking crate `github_release_rs#0.1.0`")?;
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
fn yanked_in_new_file() -> crate::Result {
    let changes = changes(index_ro()?, ":/reproduce issue #19")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.yanked().map(|v| v.name.as_str())),
        Some("allowed")
    );
    Ok(())
}

#[test]
fn unyanked_crates_recognized_as_added() -> crate::Result {
    let changes = changes(index_ro()?, ":/Unyanking crate `git2mail#0.3.2`")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.added().map(|v| v.name.as_str())),
        Some("git2mail")
    );
    Ok(())
}

#[test]
fn normalization() -> crate::Result {
    let changes = changes(index_ro()?, ":/normalize")?;
    assert_eq!(
        changes.len(),
        0,
        "normalization changes the representation, but the data itself stays the same"
    );
    Ok(())
}

fn changes(mut index: Index, revspec: &str) -> crate::Result<Vec<Change>> {
    let (prev, current) = {
        let repo = index.repository_mut();
        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        let commit = repo.rev_parse(revspec)?.single().unwrap();
        let ancestor_tree = commit
            .object()?
            .into_commit()
            .parent_ids()
            .next()
            .and_then(|parent| {
                parent
                    .object()
                    .ok()?
                    .into_commit()
                    .tree_id()
                    .ok()
                    .map(|id| id.detach())
            })
            .unwrap_or_else(|| git::hash::ObjectId::empty_tree(repo.object_hash()));
        (ancestor_tree, commit.detach())
    };
    Ok(index.changes_between_commits(prev, current)?)
}
