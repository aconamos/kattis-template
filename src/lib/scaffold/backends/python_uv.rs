use std::fs;

use anyhow::Context;

use crate::{Scaffold, ScaffoldingError, backends::PythonUv};

const PROJECT_FILE: &str = r#"""
[pyproject.toml]
name = "{contest_name}"
version = "0.1.0"
description = "Kattis template for {contest_name}"
readme = "README.md"
requires-python = ">=3.13"
dependencies = []
"""#;

const PROBLEM_FILE: &str = r#"""
import sys


def main():
    n: int = int(input())  # input() reads until first newline
    # alternatively, to read all lines, you can uncomment this line:
    # lines = [line.rstrip() for line in sys.stdin.readlines()] # rstrip removes the \n

    print("Hello, world!")


if __name__ == "__main__":
    main() 
    """#;

impl Scaffold for PythonUv {
    fn new_contest(
        &self,
        _contest_info: crate::ContestInfo,
        path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        if !fs::exists(&path).with_context(|| "error checking path")?
            && let Err(e) = fs::create_dir(&path) {
                return Err(ScaffoldingError::FileWriteError {
                    file: path,
                    source: e.into(),
                }
                .into());
            }

        if fs::read_dir(&path)?.count() != 0 {
            return Err(ScaffoldingError::NonemptyDirectoryError { directory: path }.into());
        }

        todo!()
    }

    fn new_problem(
        &self,
        _problem_info: crate::ProblemInfo,
        _path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
