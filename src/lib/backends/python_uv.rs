use std::collections::HashMap;

use crate::{
    Scaffold,
    backends::PythonUv,
    scaffold::{GraphDir, GraphFile},
};

const PROJECT_FILE: &str = r#"
[project]
name = "{contest_name}"
version = "0.1.0"
description = "Kattis template for {contest_name}"
readme = "README.md"
requires-python = ">=3.13"
dependencies = []
"#;

const PROBLEM_FILE: &str = r#"
import sys


def main():
    n: int = int(input())  # input() reads until first newline
    # alternatively, to read all lines, you can uncomment this line:
    # lines = [line.rstrip() for line in sys.stdin.readlines()] # rstrip removes the \n

    print("Hello, world!")


if __name__ == "__main__":
    main() 
    "#;

impl Scaffold for PythonUv {
    fn new_contest(&self, contest_info: crate::ContestInfo) -> anyhow::Result<GraphDir> {
        let mut root = GraphDir {
            name: "root".into(),
            child_dirs: vec![GraphDir {
                name: "src".into(),
                child_dirs: vec![],
                files: vec![GraphFile {
                    name: "$problem.py".into(),
                    contents: PROBLEM_FILE.into(),
                }],
            }],
            files: vec![GraphFile {
                name: "pyproject.toml".into(),
                contents: PROJECT_FILE.replace("{contest_name}", &contest_info.title),
            }],
        };

        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert(
            "$problem",
            contest_info
                .problems
                .iter()
                .map(|prob| prob.code.to_string())
                .collect::<Vec<_>>(),
        );

        root.expand_children_recurse(&map);

        Ok(root)
    }

    fn new_problem(&self, _problem_info: crate::ProblemInfo) -> anyhow::Result<GraphDir> {
        todo!()
    }
}
