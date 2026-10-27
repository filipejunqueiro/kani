use std::sync::LazyLock;

use regex::bytes::Regex;

static TTL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[1-9][0-9]*[mhs]\z").unwrap());

pub fn validate_ttl(ttl: &str) -> bool {
    TTL_REGEX.is_match(ttl.as_bytes())
}

#[cfg(test)]
mod tests {
    use crate::validators::ttl::validate_ttl;

    #[test]
    fn test_validate_ttl() {
        let ttl_values: [&[&str]; 2] = [
            // [0] valid values
            &[
                "1s", "1m", "1h", "5s", "30s", "59s", "10m", "999m", "24h", "999999s", "999999m",
                "9999999h",
            ],
            // [1] invalid value
            &[
                "",
                "0s",
                "0m",
                "0h",
                "-1s",
                "-5m",
                "1.5h",
                "1 s",
                "1ms",
                "1d",
                "10S",
                "10M",
                "10H",
                "s1",
                "m10",
                "h1",
                "01s",
                "001m",
                "00h",
                "1s1m",
                "1m 5s",
                "5m 0h",
                "10 seconds",
                "abc",
                "123",
                "1ss",
                "1mm",
                "1hh",
                " 5s",
                "5s ",
                "5s\n",
            ],
        ];

        for ttl in ttl_values[0] {
            assert!(validate_ttl(ttl), "Expected {ttl:?} to be valid.");
        }

        for ttl in ttl_values[1] {
            assert!(!validate_ttl(ttl), "Expected {ttl:?} to be invalid.");
        }
    }
}
