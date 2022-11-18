use std::collections::HashSet;

#[test]
fn all_aggregrated_diffs_equal_latest_version() -> git_testtools::Result {
    let ((expected, baseline_duration), _actual) =
        std::thread::scope(|s| -> git_testtools::Result<_> {
            let baseline = s
                .spawn(|| -> Result<_, crates_index::Error> {
                    let start = std::time::Instant::now();
                    let index = crates_index::Index::new_cargo_default()?;
                    let baseline = index
                        .crates()
                        .map(|c| {
                            c.versions()
                                .iter()
                                .map(|v| v.checksum().to_owned())
                                .collect::<Vec<_>>()
                        })
                        .flatten()
                        .collect::<HashSet<_>>();

                    Ok((baseline, start.elapsed()))
                })
                .join()
                .expect("no panic")?;

            Ok((baseline, ()))
        })?;

    dbg!(baseline_duration, expected.len());

    Ok(())
}
