use crates_index_diff::Change::*;
use std::collections::HashSet;

#[allow(dead_code)]
pub enum Step {
    Partitioned { size: usize },
    OnePerCommit,
}

pub fn baseline(mode: Step) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                let repo = index.repository();
                let head = repo.find_reference("refs/remotes/origin/HEAD")?.id();
                let commits = head
                    .ancestors()
                    .first_parent_only()
                    .all()?
                    .map(|id| id.map(|id| id.detach()))
                    .collect::<Result<Vec<_>, _>>()?;

                // This could be more complex, like jumping to landmarks like 'Delete crate(s)' and so forth.
                let partitions = match mode {
                    Step::Partitioned { size } => size,
                    Step::OnePerCommit => commits.len(),
                };
                let chunk_size = (commits.len() / partitions).max(1);
                let mut steps = (0..commits.len()).step_by(chunk_size).collect::<Vec<_>>();
                if *steps.last().expect("at least 1") != commits.len() - 1 {
                    steps.push(commits.len() - 1);
                }

                let mut versions = HashSet::default();
                let mut previous = None;
                let num_steps = steps.len();
                for (step, current) in steps
                    .into_iter()
                    .rev()
                    .map(|idx| commits[idx].to_owned())
                    .enumerate()
                {
                    let old = previous
                        .unwrap_or_else(|| git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1));
                    previous = Some(current);

                    let start = std::time::Instant::now();
                    let changes = index.changes_between_commits(old, current)?;
                    let num_changes = changes.len();
                    for change in changes {
                        match change {
                            Added(v) | AddedAndYanked(v) => {
                                // found a new crate, add it to the index
                                versions.insert(v.checksum.to_owned());
                            }
                            Unyanked(v) | Yanked(v) => {
                                // yanked/unyanked crates must be part of the index
                                assert!(versions.contains(&v.checksum))
                            }
                            Deleted {
                                versions: deleted, ..
                            } => {
                                // delete a yanked crate
                                for deleted_version in deleted {
                                    versions.remove(&deleted_version.checksum);
                                }
                            }
                        }
                    }
                    let elapsed = start.elapsed().as_secs_f32();
                    eprintln!(
                        "Step {} / {} and {} change(s) took {:.02}s ({:.0} changes/s)",
                        step,
                        num_steps,
                        num_changes,
                        elapsed,
                        num_changes as f32 / elapsed
                    );
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