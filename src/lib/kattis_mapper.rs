use anyhow::{Result, anyhow};
use regex::Regex;

pub const KATTIS_URL_PATTERN: &'static str =
    r"^((http(s)?://)?open.kattis.com/(contests/[a-z0-9]+/)?problems/)?(?<code>[a-z]+)$";

/// Takes a string, and verifies that it is either:
///     1. raw problem code
///     2. kattis url with problem code
/// If it succeeds, it will return the problem code. Otherwise, returns an error.
pub fn get_problem_code(input: &str) -> Result<String> {
    let re = Regex::new(KATTIS_URL_PATTERN).unwrap();
    let captures = re.captures(input);

    let Some(captures) = captures else {
        return Err(anyhow!(
            "could not extract code from input (formatting error)"
        ));
    };

    return Ok(captures["code"].to_string());
}
