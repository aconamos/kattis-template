use std::collections::HashMap;
use std::path::PathBuf;

use kattis_template::backends::{Backend, GraphDir, GraphFile};
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
    },
}

fn main() {
    let gd = GraphDir {
        name: "$among$us$among".into(),
        child_dirs: vec![],
        files: vec![GraphFile {
            name: "$among".into(),
            contents: "Hello 2".into(),
        }],
    };

    let mut parent = GraphDir {
        name: "parent".into(),
        child_dirs: vec![gd],
        files: vec![GraphFile {
            name: "$us$among".into(),
            contents: "hello!".into(),
        }],
    };

    let mut map: HashMap<&str, Vec<String>> = HashMap::new();

    map.insert("$among", vec!["a1".into(), "a2".into()]);
    map.insert("$us", vec!["u1".into(), "u2".into()]);

    parent.expand_children_recurse(&map);

    println!("{:?}", parent);

    let _ = parent.write_children_recursive(&PathBuf::from("./out"));

    // let args = Args::parse();

    // match match args.command {
    //     Commands::DownloadSamples {
    //         problem,
    //         path,
    //         write_name,
    //     } => cli::download_samples(&problem, path, write_name),
    //     Commands::InitializeContest { .. } => todo!(),
    // } {
    //     Err(e) => eprintln!("Error: {e:?}"),
    //     Ok(_) => println!("Samples downloaded!"),
    // }

    // println!(
    //     "{:?}",
    //     scraper::scrape_kattis_problem(ProblemCode::new(&args.problem).unwrap()).unwrap()
    // );
}
