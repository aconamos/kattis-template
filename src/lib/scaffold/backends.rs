use clap::ValueEnum;

mod c;
mod csharp_dotnet;
mod java_intellij;
mod python_uv;
mod rust;

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
