use crates_index_diff::Index;

mod old;
mod changes_from_objects {
    use crate::index::index_ro;

    #[test]
    #[ignore]
    fn addition() {
        let _index = index_ro();
    }
}

fn index_ro() -> Index {
    let dir = git_testtools::scripted_fixture_repo_read_only("make-index-from-parts.sh").unwrap();
    Index::from_path_or_cloned(dir).unwrap()
}
