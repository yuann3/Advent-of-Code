use regex::Regex;
use std::str::FromStr;

/// Extracts all integers (including negatives) from a string.
pub fn extract_numbers(s: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(s)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

/// Extracts all integers of generic type T from a string.
pub fn extract_ints<T: FromStr>(s: &str) -> Vec<T> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(s)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

/// Extracts only unsigned integers from a string.
pub fn extract_unsigned(s: &str) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(s)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

/// Simple scanf-like pattern matching using {} as wildcards.
/// Returns captured values as strings, or None if pattern doesn't match.
pub fn scan_pattern(s: &str, pattern: &str) -> Option<Vec<String>> {
    // Escape regex special characters except {}
    let pattern_escaped = regex::escape(pattern);
    // Replace escaped \{\} with capture group
    let pattern_regex = pattern_escaped.replace("\\{\\}", "(.*?)");
    let re = Regex::new(&format!("^{}$", pattern_regex)).ok()?;

    re.captures(s).map(|caps| {
        caps.iter()
            .skip(1) // Skip the full match
            .filter_map(|m| m.map(|m| m.as_str().to_string()))
            .collect()
    })
}

/// Splits lines into groups separated by empty lines.
pub fn split_groups(lines: &[String]) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line.clone());
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

/// Splits lines based on a custom predicate.
pub fn split_on<F>(lines: &[String], predicate: F) -> Vec<Vec<String>>
where
    F: Fn(&str) -> bool,
{
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for line in lines {
        if predicate(line) {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line.clone());
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

/// Parse or panic with a helpful error message.
pub fn parse_or_panic<T: FromStr>(s: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    s.parse().unwrap_or_else(|e| {
        panic!("Failed to parse '{}': {:?}", s, e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers("move x=10, y=-5"), vec![10, -5]);
        assert_eq!(extract_numbers("1 2 3"), vec![1, 2, 3]);
        assert_eq!(extract_numbers("no numbers"), Vec::<i64>::new());
    }

    #[test]
    fn test_extract_ints_generic() {
        let result: Vec<i32> = extract_ints("1 -2 3");
        assert_eq!(result, vec![1, -2, 3]);
    }

    #[test]
    fn test_extract_unsigned() {
        assert_eq!(extract_unsigned("1 -2 3"), vec![1, 2, 3]); // -2 becomes 2 (extracts digits)
        assert_eq!(extract_unsigned("10 20 30"), vec![10, 20, 30]);
    }

    #[test]
    fn test_scan_pattern() {
        assert_eq!(
            scan_pattern("move 10 steps", "move {} steps"),
            Some(vec!["10".to_string()])
        );
        assert_eq!(
            scan_pattern("x=5, y=10", "x={}, y={}"),
            Some(vec!["5".to_string(), "10".to_string()])
        );
        assert_eq!(scan_pattern("no match", "x={}"), None);
    }

    #[test]
    fn test_split_groups() {
        let lines = vec![
            "line1".to_string(),
            "line2".to_string(),
            "".to_string(),
            "line3".to_string(),
        ];
        let groups = split_groups(&lines);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], vec!["line1", "line2"]);
        assert_eq!(groups[1], vec!["line3"]);
    }

    #[test]
    fn test_split_on() {
        let lines = vec![
            "a".to_string(),
            "b".to_string(),
            "---".to_string(),
            "c".to_string(),
        ];
        let groups = split_on(&lines, |s| s == "---");
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], vec!["a", "b"]);
        assert_eq!(groups[1], vec!["c"]);
    }

    #[test]
    fn test_parse_or_panic() {
        assert_eq!(parse_or_panic::<i32>("42"), 42);
        assert_eq!(parse_or_panic::<f64>("3.14"), 3.14);
    }

    #[test]
    #[should_panic(expected = "Failed to parse")]
    fn test_parse_or_panic_fails() {
        parse_or_panic::<i32>("not a number");
    }
}
