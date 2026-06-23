pub mod cli;
pub mod scraping {
    pub mod scraper;
    mod scraper_util;
}
pub(crate) mod scaffold {
    pub mod backends;
}
mod types;

pub use scaffold::backends;
pub use types::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        backends::{GraphDir, get_all_possible_substitutions, get_match_idents},
        types::ProblemCode,
    };

    #[test]
    fn test_problem_code_new_full_url_contest() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("https://open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_full_url_problem() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("https://open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_http_url_contest() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("http://open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_http_url_problem() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("http://open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_no_protocol_url_contest() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_no_protocol_url_problem() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_bare_code() {
        assert_eq!(
            "hackyordering",
            ProblemCode::new("hackyordering")
                .expect("should not error on valid pattern")
                .as_ref()
        )
    }

    #[test]
    fn test_problem_code_new_empty_code_errors() {
        assert!(ProblemCode::new("").is_err())
    }

    #[test]
    fn test_problem_code_new_empty_code_url_errors() {
        assert!(ProblemCode::new("https://open.kattis.com/problems/").is_err())
    }

    #[test]
    fn test_problem_code_new_empty_code_shorturl_errors() {
        assert!(ProblemCode::new("open.kattis.com/problems/").is_err())
    }

    #[test]
    fn test_problem_code_new_empty_code_bad_format_errors() {
        assert!(ProblemCode::new("hthpghpoidn::/open.kattis.com/problems/hackyordering").is_err())
    }

    #[test]
    fn test_ident_extraction_with_one_match() {
        assert!(get_match_idents("$among/us").iter().count() == 1);
    }

    #[test]
    fn test_ident_extraction_with_many_matches() {
        assert!(
            get_match_idents("$among$us.$three$fifty5115.fasdf$aspdofij_555")
                .iter()
                .count()
                == 5
        )
    }

    #[test]
    fn test_ident_extraction_with_no_match() {
        assert!(get_match_idents("amongus").iter().count() == 0)
    }

    #[test]
    fn test_ident_extraction_with_consecutive_matches() {
        assert!(get_match_idents("$one$two$three3$4").iter().count() == 4)
    }

    #[test]
    fn bullshit() {}

    #[test]
    fn test_get_all_possible_substitutions_four_subs() {
        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert("$among", vec!["a1".into(), "a2".into()]);
        map.insert("$us", vec!["u1".into(), "u2".into()]);

        // this is my least favorite hack ever and it's only happening for the one test
        let a1: String = "a1".into();
        let a2: String = "a2".into();
        let u1: String = "u1".into();
        let u2: String = "u2".into();

        // i hate strings
        let expected_vec: Vec<Vec<(String, &String)>> = vec![
            vec![("$among".into(), &a1), ("$us".into(), &u1)],
            vec![("$among".into(), &a1), ("$us".into(), &u2)],
            vec![("$among".into(), &a2), ("$us".into(), &u1)],
            vec![("$among".into(), &a2), ("$us".into(), &u2)],
        ];

        assert!(get_all_possible_substitutions("$among$us".into(), &map) == expected_vec)
    }

    #[test]
    fn test_get_all_possible_substitutions_four_subs_with_empty() {
        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert("$among", vec!["a1".into(), "a2".into()]);
        map.insert("$us", vec!["u1".into(), "u2".into()]);

        assert!(
            get_all_possible_substitutions("$among$us$extraneous!".into(), &map)
                .iter()
                .count()
                == 4
        )
    }
}
