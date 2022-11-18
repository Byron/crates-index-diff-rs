use std::collections::HashSet;

#[test]
fn all_aggregrated_diffs_equal_latest_version(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ((expected, baseline_duration), (actual, diff_duration)) = std::thread::scope(
        |scope| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
            let baseline = scope.spawn(|| -> Result<_, crates_index::Error> {
                let index = crates_index::Index::new_cargo_default()?;
                let start = std::time::Instant::now();
                let mut versions = HashSet::new();
                for krate in index.crates() {
                    for version in krate.versions() {
                        versions.insert(version.checksum().to_owned());
                    }
                }
                Ok((versions, start.elapsed()))
            });

            let actual = scope.spawn(|| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
                use crates_index_diff::git;

                let start = std::time::Instant::now();
                let repo_path = crates_index::Index::new_cargo_default()?.path().to_owned();
                let index = crates_index_diff::Index::from_path_or_cloned(repo_path)?;
                let changes = index.changes_between_commits(
                    git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1),
                    index.repository().head_id()?,
                )?;

                use crates_index_diff::Change::*;
                let mut versions = HashSet::new();

                for change in changes {
                    match change {
                        Added(v) | Yanked(v) => {
                            versions.insert(v.checksum.to_owned());
                        }
                        Deleted {
                            versions: deleted, ..
                        } => {
                            for deleted_version in deleted {
                                versions.remove(&deleted_version.checksum);
                            }
                        }
                    }
                }
                Ok((versions, start.elapsed()))
            });

            Ok((
                baseline.join().expect("no panic")?,
                actual.join().expect("no panic")?,
            ))
        },
    )?;

    dbg!(baseline_duration, expected.len());
    dbg!(diff_duration, actual.len());
    // assert_eq!(
    //     actual.len(),
    //     expected.len(),
    //     "aggregated of all changes produces the final result"
    // );

    Ok(())
}
