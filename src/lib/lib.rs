pub mod mapper;
pub mod scraper;

#[cfg(test)]
mod tests {
    use crate::mapper::*;

    #[test]
    fn test_problem_mapper_full_url_contest() {
        assert_eq!(
            "hackyordering",
            get_problem_code("https://open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_full_url_problem() {
        assert_eq!(
            "hackyordering",
            get_problem_code("https://open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_http_url_contest() {
        assert_eq!(
            "hackyordering",
            get_problem_code("http://open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_http_url_problem() {
        assert_eq!(
            "hackyordering",
            get_problem_code("http://open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_no_protocol_url_contest() {
        assert_eq!(
            "hackyordering",
            get_problem_code("open.kattis.com/contests/jna6sj/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_no_protocol_url_problem() {
        assert_eq!(
            "hackyordering",
            get_problem_code("open.kattis.com/problems/hackyordering")
                .expect("should not have error on valid URL")
        )
    }

    #[test]
    fn test_problem_mapper_bare_code() {
        assert_eq!(
            "hackyordering",
            get_problem_code("hackyordering").expect("should not error on valid pattern")
        )
    }

    #[test]
    fn test_problem_mapper_empty_code_errors() {
        assert!(get_problem_code("").is_err())
    }

    #[test]
    fn test_problem_mapper_empty_code_url_errors() {
        assert!(get_problem_code("https://open.kattis.com/problems/").is_err())
    }

    #[test]
    fn test_problem_mapper_empty_code_shorturl_errors() {
        assert!(get_problem_code("open.kattis.com/problems/").is_err())
    }

    #[test]
    fn test_problem_mapper_empty_code_bad_format_errors() {
        assert!(get_problem_code("hthpghpoidn::/open.kattis.com/problems/hackyordering").is_err())
    }
}
