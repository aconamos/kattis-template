use std::path::PathBuf;

use anyhow::{Result, anyhow};
use thiserror::Error;

use crate::types::Sample;

#[derive(Error, Debug)]
pub enum ScaffoldingError {
    #[error("sample already exists: {sample:?} at {path:?}")]
    SampleExists { sample: Sample, path: PathBuf },
}

/// This is the collection of options available for a general backend. This means things like a Java boilerplate, a Rust boilerplate, a Python boilerplate, etc.
pub trait Scaffold {
    /// Writes a given sample to a project. This could be something like a sample1.in/sample1.out, or putting them in some in-memory structure, to be hardcoded into a test file.
    fn write_sample(&self, sample: Sample, sample_num: u8) -> Result<()>;
}
