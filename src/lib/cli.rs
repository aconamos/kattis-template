use anyhow::Result;
use std::{fs::File, io::Write, path::PathBuf};

use crate::{ProblemCode, scraper};

pub fn download_samples(problem_code: &str, path: PathBuf, write_name: bool) -> Result<()> {
    let problem_code = ProblemCode::new(problem_code)?;
    let problem_info = scraper::scrape_kattis_problem(&problem_code)?;
    let filename: &str = if write_name { &problem_code } else { "sample" };

    for (idx, sample) in problem_info.samples.iter().enumerate() {
        if let Some(input) = &sample.input {
            let mut file = File::create(path.join(format!("{filename}{idx}.in")))?;
            let _ = file.write_all(input.as_bytes());
        }

        let mut file = File::create(path.join(format!("{filename}{idx}.out")))?;
        let _ = file.write_all(sample.output.as_bytes());
    }

    Ok(())
}
