use crate::shared::Step;

mod shared;

#[cfg_attr(debug_assertions, ignore)]
#[test]
fn one_per_commit() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    shared::baseline(Step::Realistic {
        ordered_partitions: 2,
        unordered_partitions: 38,
    })
}
