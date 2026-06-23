use anyhow::{Error, Result, anyhow};
use clap::ValueEnum;
use std::ops::Deref;
use std::path::PathBuf;
use thiserror::Error;

use regex::Regex;

pub const KATTIS_URL_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^((http(s)?://)?open.kattis.com/(contests/(?<contest>[a-z0-9]+)/)?problems/)?(?<code>[a-z]+)$")
        .unwrap()
});

/// Problem code newtype. A problem code is just a string of form [a-z]+.
#[derive(Debug, Clone)]
pub struct ProblemCode(String);

/// Contest code newtype. A problem code is just a string of form [a-z0-9]+.
#[derive(Debug, Clone)]
pub struct ContestCode(String);

/// Represents a contest that has a code and problems
pub struct Contest {
    /// Code of the Contest
    pub code: String,

    /// All problems in the Contest
    pub problems: Vec<ProblemInfo>,
}

/// A sample input and output
#[derive(Debug)]
pub struct Sample {
    // sometimes, problems have no input!
    pub input: Option<String>,
    pub output: String,
}

/// The info for a given Kattis problem
#[derive(Debug)]
pub struct ProblemInfo {
    pub code: ProblemCode,
    pub title: String,
    pub samples: Vec<Sample>,
}

/// The info for a given Kattis contest
#[derive(Debug)]
pub struct ContestInfo {
    pub code: ContestCode,
    pub title: String,
    pub problems: Vec<ProblemInfo>,
}

#[derive(Error, Debug)]
pub enum ScaffoldingError {
    // todo: dedupe this from cli::CliError
    #[error("failed to create file: {file}; reason: {source:?}")]
    FileWriteError {
        file: PathBuf,

        #[source]
        source: Error,
    },

    #[error("directory was not empty: {directory}")]
    NonemptyDirectoryError { directory: PathBuf },
}

/// This is the collection of options available for a general backend. This means things like a Java boilerplate, a Rust boilerplate, a Python boilerplate, etc.
pub trait Scaffold {
    /// Initializes a new contest inside the `path` directory, creating it if it doesn't exist.
    fn new_contest(&self, contest_info: ContestInfo, path: PathBuf) -> Result<()>;

    /// Initializes a new problem inside the `path` directory, creating it if it doesn't exist.
    fn new_problem(&self, problem_info: ProblemInfo, path: PathBuf) -> Result<()>;
}

impl ProblemCode {
    /// Without checking the string, turns the input into a ProblemCode.
    pub(crate) fn new_unchecked(input: &str) -> Self {
        Self(input.to_string())
    }

    /// Creates a new ProblemCode if possible. This function accepts:
    /// 1. Full Kattis URLs, https or http
    /// 2. Partial Kattis URLs, without the protocol
    /// 3. The raw code
    ///
    /// A code is just some letters.
    pub fn new(input: &str) -> Result<Self> {
        let captures = KATTIS_URL_RE.captures(input);

        let Some(captures) = captures else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        };

        if let Some(code) = &captures.name("code") {
            return Ok(ProblemCode::new_unchecked(code.as_str()));
        } else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        }
    }
}

impl ContestCode {
    /// Without checking the string, turns the input into a ContestCode.
    pub(crate) fn new_unchecked(input: &str) -> Self {
        Self(input.to_string())
    }

    /// Creates a new ContestCode if possible. This function accepts:
    /// 1. Full Kattis URLs, https or http
    /// 2. Partial Kattis URLs, without the protocol
    /// 3. The raw code
    ///
    /// A code is just some letters.
    pub fn new(input: &str) -> Result<Self> {
        let captures = KATTIS_URL_RE.captures(input);

        let Some(captures) = captures else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        };

        if let Some(code) = &captures.name("contest") {
            return Ok(ContestCode::new_unchecked(code.as_str()));
        } else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        }
    }
}

impl Deref for ProblemCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<str> for ProblemCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a ProblemCode> for &'a str {
    fn from(value: &'a ProblemCode) -> Self {
        &value.0
    }
}
impl Deref for ContestCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<str> for ContestCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a ContestCode> for &'a str {
    fn from(value: &'a ContestCode) -> Self {
        &value.0
    }
}
