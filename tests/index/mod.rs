use crates_index_diff::Index;

mod changes_from_objects;

#[test]
fn peek_changes() {
    let mut index = index_ro().unwrap();
    index.branch_name = "main";
    assert!(
        index.last_seen_reference().is_err(),
        "marker ref doesn't exist"
    );
    let (changes, last_seen_revision) = index.peek_changes().unwrap();
    assert_eq!(
        changes.len(),
        3516,
        "all changes since the beginning of history"
    );

    let origin_main = index
        .repository()
        .find_reference("refs/remotes/origin/main")
        .unwrap();
    assert_eq!(
        last_seen_revision,
        origin_main.target().expect("direct ref"),
        "last seen reference should the latest state from the clone"
    );
    assert!(
        index.last_seen_reference().is_err(),
        "the last-seen reference has not been created"
    );
}

mod old;

fn index_ro() -> crate::Result<Index> {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )?;
    Ok(Index::from_path_or_cloned(dir.join("clone"))?)
}
