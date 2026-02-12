use anyhow::{Error, Result};
use std::{fs::File, io::Write, path::PathBuf};
use thiserror::Error;

use crate::{ProblemCode, scraper};

#[derive(Error, Debug)]
enum CliError {
    #[error("failed to create file: {file}; reason: {source:?}")]
    FileWriteError {
        file: String,

        #[source]
        source: Error,
    },

    #[error("sample download error: {source:?}")]
    SampleDownloadError {
        #[source]
        source: Error,
    },
}

/// Downloads the samples for the given ProblemCode.
///
/// This function can fail partially; if any file fails to be written, the function will still try to write all other files, but will return an Error.
pub fn download_samples(problem_code: &str, path: PathBuf, write_name: bool) -> Result<()> {
    let problem_code = ProblemCode::new(problem_code)?;
    let problem_info = scraper::scrape_kattis_problem(&problem_code)?;
    let filename: &str = if write_name { &problem_code } else { "sample" };

    let mut file_write_errors: Vec<CliError> = vec![];

    for (idx, sample) in problem_info.samples.iter().enumerate() {
        if let Some(input) = &sample.input {
            let filename = format!("{filename}{idx}.in");
            let mut file = File::create(path.join(&filename))?;
            let result = file.write_all(input.as_bytes());

            if let Err(err) = result {
                file_write_errors.push(CliError::FileWriteError {
                    file: filename,
                    source: err.into(),
                });
            }
        }

        let filename = format!("{filename}{idx}.out");
        let mut file = File::create(path.join(&filename))?;
        let result = file.write_all(sample.output.as_bytes());

        if let Err(err) = result {
            file_write_errors.push(CliError::FileWriteError {
                file: filename,
                source: err.into(),
            });
        }
    }

    // flatten file_write_errors
    Ok(())
}
