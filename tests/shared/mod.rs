use ahash::{HashMap, HashMapExt};
use crates_index_diff::Change::*;

#[allow(dead_code)]
pub enum Step {
    Partitioned {
        size: usize,
    },
    Realistic {
        /// Like `Partitioned::size, and used to have big steps until `ordered_partitions` are executed.
        unordered_partitions: usize,
        /// The amount of partitions to use for obtaining ordered changes
        ordered_partitions: usize,
    },
}

pub fn baseline(mode: Step) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ((expected, baseline_duration), (actual, diff_duration)) = std::thread::scope(
        |scope| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
            // Be sure the standard crates index is available - we can't do this in multiple threads which would otherwise
            // likely happen, causing `git2` to fail with a lock on the config file. It's curious that it has to lock it
            // in the first place.
            {
                let _index = crates_index::GitIndex::new_cargo_default()?;
            }
            let baseline = scope.spawn(|| -> Result<_, crates_index::Error> {
                let index = crates_index::GitIndex::new_cargo_default()?;
                let start = std::time::Instant::now();
                let mut versions = HashMap::new();
                for krate in index.crates() {
                    for version in krate.versions() {
                        versions.insert(version.checksum().to_owned(), version.is_yanked());
                    }
                }
                Ok((versions, start.elapsed()))
            });
            let actual = scope.spawn(|| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
                let start = std::time::Instant::now();
                let repo_path = crates_index::GitIndex::new_cargo_default()?
                    .path()
                    .to_owned();
                let index = crates_index_diff::Index::from_path_or_cloned(repo_path)?;
                let repo = index.repository();
                let head = repo
                    .find_reference("refs/remotes/origin/HEAD")?
                    .into_fully_peeled_id()?;
                let commits = head
                    .ancestors()
                    .first_parent_only()
                    .all()?
                    .map(|id| id.map(|id| id.detach()))
                    .collect::<Result<Vec<_>, _>>()?;

                enum Kind {
                    Unordered,
                    Ordered,
                }
                let (mut unordered_partitions, mut ordered_partitions) = match mode {
                    Step::Partitioned { size } => (size, 0),
                    Step::Realistic {
                        unordered_partitions,
                        ordered_partitions,
                    } => (unordered_partitions, ordered_partitions),
                };
                let chunk_size =
                    (commits.len() / (unordered_partitions + ordered_partitions)).max(1);
                let mut new_kind = || {
                    if unordered_partitions > 0 {
                        unordered_partitions -= 1;
                        Kind::Unordered
                    } else if ordered_partitions > 0 {
                        ordered_partitions -= 1;
                        Kind::Ordered
                    } else {
                        Kind::Unordered
                    }
                };
                let mut steps = (0..commits.len())
                    .step_by(chunk_size)
                    .map(|s| (s, new_kind()))
                    .collect::<Vec<_>>();
                if steps.last().expect("at least 1").0 != commits.len() - 1 {
                    steps.push((commits.len() - 1, new_kind()));
                }
                let mut versions = HashMap::default();
                let mut previous = None;
                let num_steps = steps.len();
                for (step, (current, kind)) in steps
                    .into_iter()
                    .rev()
                    .map(|(idx, kind)| (commits[idx].id, kind))
                    .enumerate()
                {
                    let old = previous
                        .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(gix::hash::Kind::Sha1));
                    previous = Some(current);

                    let start = std::time::Instant::now();
                    let changes = match kind {
                        Kind::Unordered => index.changes_between_commits(old, current)?,
                        Kind::Ordered => {
                            let (changes, actual_order) =
                                index.changes_between_ancestor_commits(old, current)?;
                            assert_eq!(
                                actual_order,
                                crates_index_diff::index::diff::Order::AsInCratesIndex,
                                "input is always correctly ordered and is commits"
                            );
                            changes
                        }
                    };
                    let num_changes = changes.len();
                    for change in changes {
                        match change {
                            Added(v) | AddedAndYanked(v) => {
                                // found a new crate, add it to the index
                                versions.insert(v.checksum.to_owned(), v.yanked);
                            }
                            Unyanked(v) | Yanked(v) => {
                                *versions
                                    .get_mut(&v.checksum)
                                    .expect("these events mean `Added*` events have been emitted") =
                                    v.yanked
                            }
                            CrateDeleted {
                                versions: deleted, ..
                            } => {
                                // delete a yanked crate
                                for deleted_version in deleted {
                                    versions.remove(&deleted_version.checksum);
                                }
                            }
                            VersionDeleted(v) => {
                                versions.remove(&v.checksum);
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
