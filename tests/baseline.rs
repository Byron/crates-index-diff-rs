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
                use crates_index_diff::Change::*;

                let start = std::time::Instant::now();
                let repo_path = crates_index::Index::new_cargo_default()?.path().to_owned();
                let index = crates_index_diff::Index::from_path_or_cloned(repo_path)?;
                let commits = index
                    .repository()
                    .find_reference("refs/remotes/origin/HEAD")?
                    .id()
                    .ancestors()
                    .first_parent_only()
                    .all()?
                    .map(|id| id.map(|id| id.detach()))
                    .collect::<Result<Vec<_>, _>>()?;

                // This could be more complex, like jumping to landmarks like 'Delete crate(s)' and so forth.
                let partitions = 3;
                let chunk_size = (commits.len() / partitions).max(1);
                let mut steps = (0..commits.len()).step_by(chunk_size).collect::<Vec<_>>();
                if *steps.last().expect("at least 1") != commits.len() - 1 {
                    steps.push(commits.len() - 1);
                }

                let mut versions = HashSet::new();
                let mut previous = None;
                for current in steps.into_iter().map(|idx| commits[idx].to_owned()) {
                    let old = previous
                        .unwrap_or_else(|| git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1));
                    previous = Some(current);

                    let changes = index.changes_between_commits(old, current)?;
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
    assert_eq!(
        actual.len(),
        expected.len(),
        "aggregated of all changes produces the final result"
    );
    assert!(actual.eq(&expected), "actual should be exactly the same");

    Ok(())
}
