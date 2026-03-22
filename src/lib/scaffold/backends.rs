use crate::types::ScaffoldingError;
use anyhow::{Context, Error, Result};
use clap::ValueEnum;
use std::fs;

use thiserror::Error;

use crate::Scaffold;

/// Represents a given backend (roughly speaking, language, but this is more general to support things like various IDEs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Backend {
    C,
    Rust,
    PythonUv,
    CsharpDotnet,
    JavaIntellij,
}

impl Scaffold for Backend {
    fn new_contest(
        &self,
        contest_info: crate::ContestInfo,
        path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn new_problem(
        &self,
        problem_info: crate::ProblemInfo,
        path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        if !fs::exists(&path).with_context(|| "error checking path")? {
            if let Err(e) = fs::create_dir(&path) {
                return Err(ScaffoldingError::FileWriteError {
                    file: path,
                    source: e.into(),
                }
                .into());
            };
        }

        todo!()
    }
}
