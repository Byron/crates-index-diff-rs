use crates_index_diff::Index;

mod old;

fn index_ro() -> Index {
    let dir = git_testtools::scripted_fixture_repo_read_only("make-index-from-parts.sh").unwrap();
    Index::from_path_or_cloned(dir).unwrap()
}
