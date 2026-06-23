use anyhow::{Context, Result};
use reqwest::StatusCode;
use scraper::{self, Html, Selector};
use thiserror::Error;

use crate::scraping::scraper_util::*;
use crate::types::{ProblemCode, ProblemInfo, Sample};
use crate::{ContestCode, ContestInfo};

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("could not find selector: {selector}")]
    MissingSelector { selector: String },

    #[error("couldn't fetch webpage for url: {}, status: {status}", url)]
    FetchError { url: String, status: StatusCode },

    #[error("tried to select one element using: {selector}, but found multiple")]
    MultipleMatches { selector: String },
}

fn get_url(url: &str) -> Result<Html> {
    let res =
        reqwest::blocking::get(url).with_context(|| "problem fetching webpage (connection)")?;
    let status = res.status();
    let res = res
        .error_for_status()
        .map_err(|_| ScraperError::FetchError {
            url: url.into(),
            status,
        })?;

    let text = res.text().with_context(|| "couldn't get text")?;

    Ok(Html::parse_document(&text))
}

/// Given a ProblemCode, searches it up, and retrieves the Samples and Title.
pub fn scrape_kattis_problem(code: &ProblemCode) -> Result<ProblemInfo> {
    let url = format!("https://open.kattis.com/problems/{}", code as &str);
    let html = get_url(&url)?;
    let kattis_info = get_kattis_problem_info(html).with_context(|| "problem scraping")?;

    Ok(ProblemInfo {
        code: code.clone(),
        title: kattis_info.0,
        samples: kattis_info.1,
    })
}

pub fn scrape_kattis_contest(code: &ContestCode) -> Result<ContestInfo> {
    let url = format!("https://open.kattis.com/contests/{}/problems", code as &str);
    let html = get_url(&url)?;
    let contest_info = get_kattis_contest_info(html).with_context(|| "problem scraping")?;
    let problems: Vec<ProblemInfo> = contest_info
        .1
        .into_iter()
        .map(|pc| scrape_kattis_problem(&pc).expect("error scraping problem from contest"))
        .collect();

    Ok(ContestInfo {
        code: code.clone(),
        title: contest_info.0,
        problems: problems,
    })
}

/// Given the HTML of a problem's webpage, fetches the ProblemInfo.
///
/// This is a function with shitty error handling. That's why it's internal.
fn get_kattis_problem_info(html: Html) -> Result<(String, Vec<Sample>)> {
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

        if data.len() == 2 {
            let input = data[0].text().next().map(|s| s.to_string());
            let output = data[1]
                .text()
                .next()
                .expect("malformed table data who cares")
                .to_string();
            Sample { input, output }
        } else {
            let output = data[0].text().next().unwrap().to_string();

            Sample {
                input: None,
                output,
            }
        }
    });

    Ok((title.to_string(), sample_info.collect()))
}

/// Given the HTML of a contest's problems page, fetches the ContestInfo.
fn get_kattis_contest_info(html: Html) -> Result<(String, Vec<ProblemCode>)> {
    let root = html.root_element();
    let problems_selector = Selector::parse("table.table2 a").unwrap();

    let title = root
        .select_one("div.flex.justify-between > h1")?
        .text()
        .fold("".to_string(), |acc, s| acc + s);
    let problems: Vec<_> = root
        .select(&problems_selector)
        .filter_map(|el| el.attr("href"))
        .map(|href| {
            href.split('/')
                .last()
                .expect("extracted href has no slashes???")
        })
        .map(|problem_code| ProblemCode::new(problem_code).expect("problem code from href is bad"))
        .collect();

    Ok((title, problems))
}
