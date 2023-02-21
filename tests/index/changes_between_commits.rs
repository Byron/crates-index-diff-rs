use crate::index::index_ro;
use crates_index_diff::index::diff::Order;
use crates_index_diff::{Change, CrateVersion, Index};

#[test]
fn directory_deletions_are_not_picked_up() -> crate::Result {
    let changes = changes(index_ro()?, ":/reproduce issue #20")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes.first().and_then(|c| c.deleted().map(|t| t.0)),
        Some("allowed")
    );
    Ok(())
}

#[test]
fn ancestor_commits_retain_order() -> crate::Result {
    let index = index_ro()?;
    let repo = index.repository();
    let from = repo.rev_parse_single("@^{/Yanking crate `gitten#0.3.1`}~1")?;
    let to = repo.rev_parse_single(":/Yanking crate `gitten#0.3.0`")?;
    let (changes, order) = index.changes_between_ancestor_commits(from, to)?;

    assert_eq!(order, Order::AsInCratesIndex, "both commits are connected");
    assert_eq!(
        changes.len(),
        2,
        "we did specify one more than we needed as the `from` commit would otherwise not be included (hence `~1`)"
    );

    assert_eq!(
        changes[0].yanked().expect("yanked").version,
        "0.3.1",
        "this goes against ascending order, but is what's recorded in the crates index"
    );

    assert_eq!(changes[1].yanked().expect("yanked").version, "0.3.0");
    Ok(())
}

#[test]
fn updates_before_yanks_are_picked_up() -> crate::Result {
    let index = index_ro()?;
    let repo = index.repository();
    let from = repo.rev_parse_single("@^{/updating ansi-color-codec 0.3.11}~1")?;
    let to = repo.rev_parse_single("@^{/yanking ansi-color-codec 0.3.5}")?;
    let mut changes = index.changes_between_commits(from, to)?;

    assert_eq!(changes.len(), 3, "1 update and 2 yanks");
    changes.sort_by_key(|change| change.versions()[0].version.clone());
    assert_eq!(changes[0].added().expect("first updated").version, "0.3.11");
    assert_eq!(changes[1].yanked().expect("second yanked").version, "0.3.4");
    assert_eq!(changes[2].yanked().expect("third yanked").version, "0.3.5");

    let (mut changes, order) = index.changes_between_ancestor_commits(from, to)?;
    assert_eq!(
        order,
        Order::AsInCratesIndex,
        "we provided commits, so ancestry should pan out"
    );

    assert_eq!(changes.len(), 3, "1 update and 2 yanks");
    changes.sort_by_key(|change| change.versions()[0].version.clone());
    assert_eq!(changes[0].added().expect("first updated").version, "0.3.11");
    assert_eq!(changes[1].yanked().expect("second yanked").version, "0.3.4");
    assert_eq!(changes[2].yanked().expect("third yanked").version, "0.3.5");
    Ok(())
}

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
    let changes = changes(index_ro()?, "@^{/Delete crates}")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes.first().and_then(|c| c.deleted().map(|t| t.0)),
        Some("girl")
    );
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
fn unyanked_crates_recognized() -> crate::Result {
    let changes = changes(index_ro()?, ":/Unyanking crate `git2mail#0.3.2`")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.unyanked().map(|v| v.name.as_str())),
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
        let commit = repo
            .rev_parse(revspec)?
            .single()
            .expect("well-known revspec always exists in test setup");
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
            .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()));
        (ancestor_tree, commit.detach())
    };
    Ok(index.changes_between_commits(prev, current)?)
}
