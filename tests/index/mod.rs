use crates_index_diff::Index;

mod old;
mod changes_from_objects {
    use crate::index::index_ro;

    #[test]
    fn addition() {
        let _index = index_ro();
    }
}

fn index_ro() -> Index {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )
    .unwrap();
    Index::from_path_or_cloned(dir.join("base")).unwrap()
}
