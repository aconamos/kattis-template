use anyhow::Result;
use std::path::PathBuf;

use crate::{ProblemCode, scraper};

pub fn download_samples(problem_code: &str, path: PathBuf) -> Result<()> {
    let problem_code = ProblemCode::new(problem_code)?;
    let problem_info = scraper::scrape_kattis_problem(problem_code)?;

    println!("{:?}", problem_info);

    Ok(())
}
