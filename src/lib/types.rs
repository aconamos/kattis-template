use std::ops::Deref;

/// Problem code newtype. A problem code is just a string of form [a-z]+.
pub struct ProblemCode(String);

/// A sample input and output
pub struct Sample {
    input: String,
    output: String,
}

/// The info for a given Kattis problem
pub struct ProblemInfo {
    code: String,
    description: String,
    samples: Vec<Sample>,
}

impl ProblemCode {
    /// Without checking the string, turns the input into a ProblemCode.
    pub(crate) fn new_unchecked(input: &str) -> Self {
        Self(input.to_string())
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
