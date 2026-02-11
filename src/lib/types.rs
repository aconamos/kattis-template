use anyhow::{Result, anyhow};
use std::ops::Deref;

use regex::Regex;

pub const KATTIS_URL_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^((http(s)?://)?open.kattis.com/(contests/[a-z0-9]+/)?problems/)?(?<code>[a-z]+)$")
        .unwrap()
});

/// Problem code newtype. A problem code is just a string of form [a-z]+.
#[derive(Debug, Clone)]
pub struct ProblemCode(String);

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

        return Ok(ProblemCode::new_unchecked(&captures["code"]));
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
