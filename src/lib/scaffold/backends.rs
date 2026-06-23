use crate::types::ScaffoldingError;
use anyhow::{Context, Error, Result};
use clap::ValueEnum;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, path::PathBuf};

mod c;
mod csharp_dotnet;
mod java_intellij;
mod python_uv;
mod rust;

pub const IDENT_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\$[a-zA-Z0-9]+").unwrap());

/// Represents a given backend (roughly speaking, language, but this is more general to support things like various IDEs)
/// These backends each have a corresponding struct that may contain settings information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Backend {
    C,
    Rust,
    PythonUv,
    CsharpDotnet,
    JavaIntellij,
}

//
// Backend Structs
//
// In the future, these may contain any information about their project settings/configurations.
// For example, which standard of the C language is being used. Right now, there are just zero-sized
// structs for the impl of Scaffold.
//

/// C backend with a given standard
pub struct C {}

/// Rust backend
pub struct Rust {}

/// Python backend, managed by uv
pub struct PythonUv {}

/// C# backend, using Rider
pub struct CsharpDotnet {}

/// Java backend, using IntelliJ
pub struct JavaIntellij {}

/// Uses the identifier regex [`IDENT_RE`] to extract every match in the string.
pub(crate) fn get_match_idents(src: &str) -> Vec<String> {
    IDENT_RE
        .find_iter(src)
        .map(|x| x.as_str().to_string())
        .collect()
}

/// Given a source string containing identifiers, and a map containing substitutions for those identifiers,
/// returns a vector containing vectors of tuples that identify every possible combination of substitutions.
///
/// If a given identifier doesn't have a substitution, it is left alone.
pub(crate) fn get_all_possible_substitutions(
    src_str: &str,
    map: &HashMap<&str, Vec<String>>,
) -> Vec<String> {
    let identifiers: Vec<String> = get_match_idents(src_str);
    let mut strs: Vec<String> = vec![src_str.into()];
    let mut identifiers_left = identifiers.len();

    while identifiers_left > 0 {
        let mut temp: Vec<String> = vec![];

        for str in strs {
            let current_identifier = &identifiers[identifiers.len() - identifiers_left];

            let Some(subs) = map.get(current_identifier as &str) else {
                temp.push(str);
                continue;
            };

            for sub in subs {
                temp.push(str.replacen(current_identifier, sub, 1));
            }
        }

        strs = temp;
        identifiers_left -= 1;
    }

    strs
}

#[derive(Debug, Clone)]
pub struct GraphDir {
    pub name: String,
    pub child_dirs: Vec<GraphDir>,
    pub files: Vec<GraphFile>,
}

impl GraphDir {
    pub fn new(name: String) -> Self {
        GraphDir {
            name,
            child_dirs: vec![],
            files: vec![],
        }
    }

    pub fn expand_children(&mut self, map: &HashMap<&str, Vec<String>>) {
        let dirs: Vec<_> = self
            .child_dirs
            .iter()
            .map(|dir| {
                get_all_possible_substitutions(&dir.name, map)
                    .into_iter()
                    .map(|name| GraphDir {
                        name,
                        ..dir.clone()
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        self.child_dirs = dirs;

        let files: Vec<_> = self
            .files
            .iter()
            .map(|file| {
                get_all_possible_substitutions(&file.name, map)
                    .into_iter()
                    .map(|name| GraphFile {
                        name,
                        ..file.clone()
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        self.files = files;
    }

    pub fn expand_children_recurse(&mut self, map: &HashMap<&str, Vec<String>>) {
        self.expand_children(map);

        for dir in &mut self.child_dirs {
            dir.expand_children_recurse(map);
        }
    }

    pub fn write_children(&self, path: &PathBuf) -> Result<()> {
        for dir in &self.child_dirs {
            let mut path = path.clone();
            path.push(&dir.name);
            fs::create_dir(path)?;
        }

        for file in &self.files {
            let mut path = path.clone();
            path.push(&file.name);
            fs::write(path, &file.contents)?;
        }

        Ok(())
    }

    pub fn write_children_recursive(&self, path: &PathBuf) -> Result<()> {
        self.write_children(path)?;

        for dir in &self.child_dirs {
            let mut path = path.clone();
            path.push(&dir.name);
            dir.write_children_recursive(&path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GraphFile {
    pub name: String,
    pub contents: String,
}
