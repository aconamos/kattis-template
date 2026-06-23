pub mod cli;
pub mod scraping {
    pub mod scraper;
    mod scraper_util;
}
pub mod scaffold;
mod types;

pub use scaffold::backends;
pub use types::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        scaffold::{GraphDir, get_all_possible_substitutions, get_match_idents},
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
    fn test_ident_extraction_with_repeat() {
        assert!(get_match_idents("$one$two$one").iter().count() == 3)
    }

    #[test]
    fn bullshit() {}

    #[test]
    fn test_get_all_possible_substitutions_four_subs() {
        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert("$among", vec!["a1".into(), "a2".into()]);
        map.insert("$us", vec!["u1".into(), "u2".into()]);

        let expected_vec: Vec<String> = vec!["a1u1", "a1u2", "a2u1", "a2u2"]
            .into_iter()
            .map(|x| x.into())
            .collect();

        assert_eq!(
            get_all_possible_substitutions("$among$us".into(), &map),
            expected_vec
        )
    }

    #[test]
    fn test_get_all_possible_substitutions_four_subs_with_empty() {
        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert("$among", vec!["a1".into(), "a2".into()]);
        map.insert("$us", vec!["u1".into(), "u2".into()]);

        let expected_vec: Vec<String> = vec![
            "a1u1$extraneous",
            "a1u2$extraneous",
            "a2u1$extraneous",
            "a2u2$extraneous",
        ]
        .into_iter()
        .map(|x| x.into())
        .collect();

        assert_eq!(
            get_all_possible_substitutions("$among$us$extraneous".into(), &map),
            expected_vec
        )
    }

    #[test]
    fn test_get_all_possible_substitutions_four_subs_with_duplicate() {
        let mut map: HashMap<&str, Vec<String>> = HashMap::new();

        map.insert("$among", vec!["a1".into(), "a2".into()]);

        let expected_vec: Vec<String> = vec!["a1a1", "a1a2", "a2a1", "a2a2"]
            .into_iter()
            .map(|x| x.into())
            .collect();

        assert_eq!(
            get_all_possible_substitutions("$among$among".into(), &map),
            expected_vec
        )
    }
}
