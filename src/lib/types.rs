use anyhow::{Result, anyhow};
use std::ops::Deref;

use regex::Regex;

use crate::scaffold::GraphDir;

pub const KATTIS_PROBLEM_URL_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^((http(s)?://)?open.kattis.com/(contests/(?<contest>[a-z0-9]+)/)?problems/)?(?<code>[a-z]+)$")
        .unwrap()
});

pub const KATTIS_CONTEST_URL_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(
        r"^((http(s)?://)?open\.kattis\.com/contests/)?(?<contest>[a-z0-9]+)(/problems(/[a-z]+)?)?$",
    )
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

/// This is the collection of options available for a general backend. This means things like a Java boilerplate, a Rust boilerplate, a Python boilerplate, etc.
pub trait Scaffold {
    /// Generate a directory structure for the given contest info. The returned node represents the directory containing the project.
    fn new_contest(&self, contest_info: ContestInfo) -> Result<GraphDir>;

    /// Generate a directory structure for the given problem info. The returned node represents the directory containing the project.
    fn new_problem(&self, problem_info: ProblemInfo) -> Result<GraphDir>;
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
        let captures = KATTIS_PROBLEM_URL_RE.captures(input);

        let Some(captures) = captures else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        };

        if let Some(code) = &captures.name("code") {
            Ok(ProblemCode::new_unchecked(code.as_str()))
        } else {
            Err(anyhow!(
                "could not extract code from input (formatting error)"
            ))
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
        let captures = KATTIS_CONTEST_URL_RE.captures(input);

        let Some(captures) = captures else {
            return Err(anyhow!(
                "could not extract code from input (formatting error)"
            ));
        };

        if let Some(code) = &captures.name("contest") {
            Ok(ContestCode::new_unchecked(code.as_str()))
        } else {
            Err(anyhow!(
                "could not extract code from input (formatting error)"
            ))
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
