pub mod scraper;
mod scraper_util;
mod types;

pub use types::ProblemCode;

#[cfg(test)]
mod tests {
    use crate::types::ProblemCode;

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
}
