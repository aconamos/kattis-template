use kattis_template::ProblemCode;
use kattis_template::scraper;

fn main() {
    println!(
        "{:?}",
        scraper::scrape_kattis_problem(
            ProblemCode::new("open.kattis.com/problems/lvable").unwrap()
        )
        .unwrap()
    );
}
