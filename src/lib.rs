use core::str::FromStr;

pub fn pattern_separated_values<T: FromStr>(input: &str, pattern: char) -> Vec<T> {
    input
        .trim()
        .split(pattern)
        .filter_map(|s| s.parse().ok())
        .collect()
}
