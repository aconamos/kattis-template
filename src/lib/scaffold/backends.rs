use crate::types::ScaffoldingError;
use anyhow::{Context, Error, Result};
use clap::ValueEnum;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

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
pub(crate) fn get_all_possible_substitutions<'a>(
    src_str: &str,
    map: &'a HashMap<&str, Vec<String>>,
) -> Vec<Vec<(String, &'a String)>> {
    let idents = get_match_idents(&src_str);

    // the goal here is to turn each identifier into a list of tuples where each
    // tuple has the identifier and one of the possible substitutions, for every
    // possible substitution. we'll then take the cross product
    //
    // the reason we will keep a tuple including the original identifier
    // is because it makes taking the cross product a lot easier
    let substitution_list: Vec<_> = idents
        .into_iter()
        .filter_map(|ident| {
            // easy return if there's no substitutions
            let Some(subs) = map.get(ident.as_str()) else {
                return None;
            };

            Some(
                subs.into_iter()
                    // just turn each substitution into its identifier with the sub
                    // todo: can we get rid of this clone?
                    .map(|sub| (ident.clone(), sub))
                    // keep that as a vec or the final one will be weird
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let all_possible_substitutions: Vec<_> = substitution_list
        .into_iter()
        .multi_cartesian_product()
        .into_iter()
        .collect();
    println!("{:?}", all_possible_substitutions);
    all_possible_substitutions
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

    pub fn write_down(&self, map: &HashMap<&str, Vec<String>>) -> Result<()> {
        let dirs: Vec<_> = self
            .child_dirs
            .iter()
            .map(move |dir| {
                get_all_possible_substitutions(&dir.name, map)
                    .iter()
                    .map(|sublist| {
                        let mut thingamajig = dir.name.clone();

                        for sub in sublist {
                            thingamajig = thingamajig.replace(&sub.0, sub.1);
                        }

                        GraphDir {
                            name: thingamajig,
                            ..dir.clone()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        println!("{:?}", dirs);
        return Ok(());

        for dir in &self.child_dirs {}
        todo!("Write the children directories");
        // for dir in dirs {
        //     // The way that this is called, we expect to be in an empty directory
        //     fs::create_dir(dir.name)?;
        // }
        todo!("Expand the child files");
        let files = self.files;
        todo!("Write the child files");
        for file in files {
            todo!("expand content using hashmap")
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GraphFile {
    pub name: String,
    pub contents: String,
}

/// Simple abstraction over a file that contains templated strings (i.e. the strings can have substitutions in them)
pub struct TemplatedFile {
    /// Tempalted name of the file
    name: String,

    /// Templated contents of the file
    contents: String,
}

impl TemplatedFile {
    /// Substitutes a given string for all instances of the identifier in the file's contents and name,
    /// returning a new TemplatedFile.
    pub fn make_substition(&self, ident: &str, sub: &str) -> TemplatedFile {
        TemplatedFile {
            name: self.name.replace(ident, sub),
            contents: self.contents.replace(ident, sub),
        }
    }
}
