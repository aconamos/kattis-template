use anyhow::{Context, Result};
use reqwest::StatusCode;
use scraper::{self, Html, Selector};
use thiserror::Error;

use crate::scraper_util::*;
use crate::types::{ProblemCode, ProblemInfo, Sample};

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("could not find selector: {selector}")]
    MissingSelector { selector: String },

    #[error("couldn't fetch webpage for problem code: {}, status: {status}", code.as_ref())]
    FetchError {
        code: ProblemCode,
        status: StatusCode,
    },

    #[error("tried to select one element using: {selector}, but found multiple")]
    MultipleMatches { selector: String },
}

/// Given a ProblemCode, searches it up, and retrieves the Samples and Title.
pub fn scrape_kattis_problem(code: ProblemCode) -> Result<ProblemInfo> {
    let url = format!("https://open.kattis.com/problems/{}", code.as_ref());
    let res =
        reqwest::blocking::get(url).with_context(|| "problem fetching webpage (connection)")?;
    let status = res.status();
    let res = res
        .error_for_status()
        .map_err(|_| ScraperError::FetchError {
            code: code.clone(),
            status,
        })?;

    let text = res.text().with_context(|| "couldn't get text")?;

    let html = Html::parse_document(&text);

    let kattis_info = get_kattis_info(html).with_context(|| "problem scraping")?;

    Ok(ProblemInfo {
        code,
        title: kattis_info.0,
        samples: kattis_info.1,
    })
}

/// Given the HTML of a problem's webpage, fetches the ProblemInfo.
///
/// This is a function with shitty error handling. That's why it's internal.
fn get_kattis_info(html: Html) -> Result<(String, Vec<Sample>)> {
    let root = html.root_element();
    let ts_sel = Selector::parse("table.sample").unwrap();

    let article = root.select_one("article.book-page")?;
    let heading = article.select_one("h1.book-page-heading")?;
    let samples = article.select(&ts_sel);

    let title = heading.text().collect::<Vec<_>>()[0];

    let sample_info = samples.map(|sample| {
        let data: Vec<_> = sample
            .select(&Selector::parse("td > pre").unwrap())
            .collect();

        let input = data[0]
            .text()
            .nth(0)
            .expect("malformed table data who cares")
            .to_string();
        let output = data[1]
            .text()
            .nth(0)
            .expect("malformed table data who cares")
            .to_string();

        Sample { input, output }
    });

    Ok((title.to_string(), sample_info.collect()))
}
