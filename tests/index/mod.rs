use crates_index_diff::index::diff::Order;
use crates_index_diff::Index;
use gix::refs::transaction::PreviousValue;
use gix_testtools::tempfile::TempDir;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

mod changes_between_commits;

const NUM_CHANGES_SINCE_EVER: usize = 3521;

#[test]
fn peek_changes() -> crate::Result {
    let mut index = index_ro()?;
    index.branch_name = "main";
    for order in [Order::ImplementationDefined, Order::AsInCratesIndex] {
        assert!(
            index.last_seen_reference().is_err(),
            "marker ref doesn't exist"
        );
        let (changes, last_seen_revision) = index.peek_changes_with_options(
            gix::progress::Discard,
            &AtomicBool::default(),
            order,
        )?;
        assert_eq!(
            changes.len(),
            NUM_CHANGES_SINCE_EVER,
            "all changes since the beginning of history"
        );

        let origin_main = index
            .repository()
            .find_reference("refs/remotes/origin/main")?;
        assert_eq!(
            last_seen_revision,
            origin_main.id(),
            "last seen reference should the latest state from the clone"
        );
        assert!(
            index.last_seen_reference().is_err(),
            "the last-seen reference has not been created"
        );
    }
    Ok(())
}

#[test]
fn clone_if_needed() {
    let tmp = TempDir::new().unwrap();
    let no_interrupt = &AtomicBool::default();
    Index::from_path_or_cloned_with_options(
        tmp.path(),
        gix::progress::Discard,
        no_interrupt,
        clone_options(),
    )
    .expect("successful clone to be created");
    Index::from_path_or_cloned_with_options(
        tmp.path(),
        gix::progress::Discard,
        no_interrupt,
        clone_options(),
    )
    .expect("second instance re-uses existing clone");
}

#[test]
fn changes_since_last_fetch() {
    let (mut index, _tmp) = index_rw().unwrap();
    let repo = index.repository();
    assert!(index.last_seen_reference().is_err(), "no marker exists");
    let num_changes_since_first_commit = index.fetch_changes().unwrap().len();
    assert_eq!(
        num_changes_since_first_commit, NUM_CHANGES_SINCE_EVER,
        "all changes since ever"
    );
    let mut marker = index
        .last_seen_reference()
        .expect("must be created/update now");
    let remote_main = repo.find_reference("refs/remotes/origin/main").unwrap();
    assert_eq!(
        marker.target(),
        remote_main.target(),
        "we are updated to the most recent known version of the remote"
    );

    // reset to previous one
    marker
        .set_target_id(
            repo.rev_parse(format!("{}~1", index.seen_ref_name).as_str())
                .unwrap()
                .single()
                .unwrap(),
            "resetting to previous commit",
        )
        .expect("reset success");
    let num_seen_after_reset = index.fetch_changes().unwrap().len();
    assert_eq!(
        index.last_seen_reference().unwrap().target(),
        remote_main.target(),
        "seen branch was updated again"
    );
    assert_eq!(
        num_seen_after_reset, 2,
        "normalization has no changes, but the commit before has one, and apparently the version-deleted commit also adds one here"
    );

    assert_eq!(
        index.fetch_changes().unwrap().len(),
        0,
        "nothing if there was no change"
    );

    // now the remote has squashed their history, we should still be able to get the correct changes.
    let repo_name = "local";
    {
        let git_dir = repo.git_dir().to_owned();
        let mut config = index.repository_mut().config_snapshot_mut();
        // TODO: use `remote.save_as_to()` here, requires a way to get the mutable repo ref again.
        config
            .set_raw_value(
                "remote",
                Some(repo_name.into()),
                "url",
                git_dir.to_str().unwrap(),
            )
            .unwrap();
        config
            .set_raw_value(
                "remote",
                Some(repo_name.into()),
                "fetch",
                "+refs/heads/*:refs/remotes/local/*",
            )
            .unwrap();
    }
    index.remote_name = Some(repo_name.into());
    index
        .repository()
        .reference(
            "refs/heads/main",
            index
                .repository()
                .rev_parse("origin/squashed")
                .unwrap()
                .single()
                .unwrap(),
            PreviousValue::Any,
            "adjust to simulate remote with new squashed history",
        )
        .unwrap();
    let changes = index.fetch_changes().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes
            .first()
            .and_then(|c| c.added().map(|v| (v.name.as_str(), v.version.as_str()))),
        Some(("git-repository", "1.0.0")),
        "there was just one actual changes compared to the previous state"
    );
}

fn index_ro() -> crate::Result<Index> {
    let dir = fixture_dir()?;
    Ok(Index::from_path_or_cloned(dir.join("clone"))?)
}

fn index_rw() -> crate::Result<(Index, TempDir)> {
    let tmp = TempDir::new().unwrap();
    let mut index = Index::from_path_or_cloned_with_options(
        tmp.path(),
        gix::progress::Discard,
        &AtomicBool::default(),
        clone_options(),
    )?;
    index.branch_name = "main";
    Ok((index, tmp))
}

fn fixture_dir() -> crate::Result<PathBuf> {
    gix_testtools::scripted_fixture_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )
}

fn clone_options() -> crates_index_diff::index::CloneOptions {
    crates_index_diff::index::CloneOptions {
        url: fixture_dir().unwrap().join("base").display().to_string(),
    }
}
