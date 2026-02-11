use anyhow::Result;
use scraper::{ElementRef, Selector};

use crate::scraper::ScraperError;

/// Scraping helper trait
pub trait SelectOne {
    /// Selects one thing and errors if there isn't exactly one
    fn select_one(&self, query: &str) -> Result<ElementRef<'_>>;
}

impl<'a> SelectOne for ElementRef<'a> {
    fn select_one(&self, query: &str) -> Result<ElementRef<'_>> {
        let selector = Selector::parse(query).unwrap();
        let mut selection = self.select(&selector);
        let first = selection.nth(0);

        match first {
            None => Err(ScraperError::MissingSelector {
                selector: query.to_string(),
            }
            .into()),
            Some(thing) => match selection.nth(0) {
                None => Ok(thing),
                Some(_) => Err(ScraperError::MultipleMatches {
                    selector: query.to_string(),
                }
                .into()),
            },
        }
    }
}
