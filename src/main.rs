use kattis_template::mapper;

fn main() {
    println!(
        "{:?}",
        kattis_template::scraper::scrape_kattis_problem(
            mapper::get_problem_code("https://open.kattis.com/problems/lvable").unwrap(),
        )
        .unwrap()
    );
}
