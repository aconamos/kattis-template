use std::path::PathBuf;

use kattis_template::backends::Backend;
use kattis_template::cli;

use clap::{Parser, Subcommand};

/// Generate the boilerplate to jump right into solving Kattis problems.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

/// Various subcommmands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Download the samples for a given problem.
    DownloadSamples {
        /// The problem to download samples for. This could be a Kattis URL or just the problem code.
        problem: String,

        /// Path to the directory to download the samples at.
        #[arg(long, value_name = "directory", default_value = ".")]
        path: PathBuf,

        /// Whether to write the samples like `sample1.out` or `problem_code.out`.
        #[arg(long, default_value_t = false)]
        write_name: bool,
    },

    /// Initializes a project by writing boilerplate for each problem into a new directory.
    InitializeContest {
        /// The contest. This could be a Kattis URL or just the contest code.
        contest: String,

        /// The language to initialize.
        #[arg(long)]
        language: Backend,

        /// Path to directory to initialize the contest project at. Defaults to a new
        /// directory with the contest code.
        #[arg(long, value_name = "directory")]
        path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match match args.command {
        Commands::DownloadSamples {
            problem,
            path,
            write_name,
        } => cli::download_samples(&problem, path, write_name),
        Commands::InitializeContest {
            contest,
            language,
            path,
        } => cli::initialize_contest(&contest, language, path),
    } {
        Err(e) => eprintln!("Error: {e:?}"),
        Ok(_) => println!("Samples downloaded!"),
    }
}
