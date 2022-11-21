use crate::shared::Step;
mod shared;

#[cfg_attr(debug_assertions, ignore)]
#[test]
fn big_steps() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    shared::baseline(Step::Partitioned { size: 4 })
}
