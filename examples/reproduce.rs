use crates_index_diff::{Change, Index};

fn main() {
    let index = Index::from_path_or_cloned(
        std::env::args_os()
            .nth(1)
            .expect("First argument is the location of the crates.io index"),
    )
    .unwrap();
    let changes = index.changes("e36aee0~1", "e36aee0").unwrap();
    for change in changes {
        dbg!(&change);
        match &change {
            Change::Added(cv) if cv.name == "dl" => {}
            Change::Yanked(cv) if cv.name == "dl" => {}
            Change::Deleted { name, .. } if name == "dl" => {}
            _ => continue,
        }

        eprintln!("{:?}", change);
    }
}
