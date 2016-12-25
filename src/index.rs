use std::path::PathBuf;

pub struct Index {}

impl Index {
    pub fn from_path<P>(path: P) -> Result<Index, ()>
        where P: Into<PathBuf> {
        Err(())
    }
}