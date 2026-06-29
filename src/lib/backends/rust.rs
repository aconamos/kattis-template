use std::collections::HashMap;

use crate::{
    Scaffold,
    backends::Rust,
    scaffold::{GraphDir, GraphFile},
};

const PROJECT_FILE: &str = r#"
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("couldn't read stdin!");
}
"#;

const CARGO_TOML: &str = r#"
[package]
name = "{contest_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;

const BINARY_TARGET: &str = r#"
[[bin]]
name = "{problem_name}"
test = false
bench = false
"#;

impl Scaffold for Rust {
    fn new_contest(&self, contest_info: crate::ContestInfo) -> anyhow::Result<GraphDir> {
        let mut root = GraphDir {
            name: "root".into(),
            child_dirs: vec![],
            files: vec![],
        };

        let src = GraphDir {
            name: "src".into(),
            child_dirs: vec![GraphDir {
                name: "bin".into(),
                child_dirs: vec![],
                files: vec![GraphFile {
                    name: "$problem.rs".into(),
                    contents: PROJECT_FILE.to_owned(),
                }],
            }],
            files: vec![],
        };

        let binary_targets = contest_info
            .problems
            .iter()
            .map(|prob| BINARY_TARGET.replace("{problem_name}", &prob.code))
            .fold(String::new(), |acc, s| acc + "\n" + &s);

        let cargo_toml = GraphFile {
            name: "Cargo.toml".into(),
            contents: (CARGO_TOML.replace("{contest_name}", &contest_info.code) + &binary_targets),
        };

        // TODO: Make decision on including .gitignore

        // let gitignore = GraphFile {
        //     name: ".gitignore".into(),
        //     contents: "/target".into(),
        // };

        root.files.push(cargo_toml);
        // root.files.push(gitignore);
        root.child_dirs.push(src);

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

    fn new_problem(&self, problem_info: crate::ProblemInfo) -> anyhow::Result<GraphDir> {
        let mut root = GraphDir {
            name: "root".into(),
            child_dirs: vec![],
            files: vec![],
        };

        let src = GraphDir {
            name: "src".into(),
            child_dirs: vec![],
            files: vec![GraphFile {
                name: "main.rs".into(),
                contents: PROJECT_FILE.to_owned(),
            }],
        };

        let cargo_toml = GraphFile {
            name: "Cargo.toml".into(),
            contents: CARGO_TOML.replace("{contest_name}", &problem_info.code),
        };

        // TODO: Make decision on including .gitignore

        // let gitignore = GraphFile {
        //     name: ".gitignore".into(),
        //     contents: "/target".into(),
        // };

        root.files.push(cargo_toml);
        // root.files.push(gitignore);
        root.child_dirs.push(src);

        Ok(root)
    }
}
