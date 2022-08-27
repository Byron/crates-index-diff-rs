use crates_index_diff::Index;

mod changes_from_objects;
mod old;

fn index_ro() -> crate::Result<Index> {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )?;
    Ok(Index::from_path_or_cloned(dir.join("base"))?)
}
