use std::path::PathBuf;

use kattis_template::ProblemCode;
use kattis_template::cli;
use kattis_template::scraper;

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
    },
}

fn main() {
    let args = Args::parse();

    match match args.command {
        Commands::DownloadSamples { problem, path } => cli::download_samples(&problem, path),
    } {
        Err(e) => eprintln!("Error: {e:?}"),
        Ok(_) => println!("Samples downloaded!"),
    }

    // println!(
    //     "{:?}",
    //     scraper::scrape_kattis_problem(ProblemCode::new(&args.problem).unwrap()).unwrap()
    // );
}
