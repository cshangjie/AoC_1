use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;
use regex::Regex;

fn find_first_last_digit_or_substring<'a>(
    s: &'a str,
    hashmap: &'a HashMap<&str, &str>,
) -> (Option<&'a str>, Option<&'a str>) {
    let pattern = Regex::new(r#"[1-9]|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    let mut matches: VecDeque<&'a str> = pattern.find_iter(s).map(|mat| mat.as_str()).collect();

    let replaced_first = matches.pop_front().map_or(None, |first| hashmap.get(first).copied().or(Some(first)));
    let replaced_last = matches.pop_back().map_or(None, |last| hashmap.get(last).copied().or(Some(last)));

    (replaced_first, replaced_last)
}

fn main() -> io::Result<()> {
    let file_path = "input.txt"; // Update with your file path
    let hashmap: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ].iter().cloned().collect();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize running total
    let mut total: i32 = 0;

    // Process each line in the file
    for line in reader.lines() {
        let input_str = line?;
        let (first, last) = find_first_last_digit_or_substring(&input_str, &hashmap);

        match (first, last) {
            (Some(first), Some(last)) => {
                let result = format!("{}{}", first, last);
                let result_num: i32 = result.parse().unwrap_or_else(|_| {
                    println!("Failed to parse result: {}", result);
                    0
                });
                total += result_num;
            }
            (Some(first), None) => {
                let result_num: i32 = format!("{}", first).parse().unwrap_or_else(|_| {
                    println!("Failed to parse result: {}", first);
                    0
                });
                total += result_num;
            }
            (None, Some(last)) => {
                let result_num: i32 = format!("{}", last).parse().unwrap_or_else(|_| {
                    println!("Failed to parse result: {}", last);
                    0
                });
                total += result_num;
            }
            (None, None) => {
            }
        }
    }

    // Print the running total
    println!("Running Total: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_last_digit_or_substring() {
        let hashmap: HashMap<&str, &str> = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ].iter().cloned().collect();

        let input_str = "7b";
        let (first, last) = find_first_last_digit_or_substring(input_str, &hashmap);
        assert_eq!(first, Some("7"));
        assert_eq!(last, None);

        // Test case 2: Only first replacement is found
        let input_str = "7";
        let (first, last) = find_first_last_digit_or_substring(input_str, &hashmap);
        assert_eq!(first, Some("7"));
        assert_eq!(last, None);

        // Test case 4: Neither first nor last replacement is found
        let input_str = "";
        let (first, last) = find_first_last_digit_or_substring(input_str, &hashmap);
        assert_eq!(first, None);
        assert_eq!(last, None);
    }
}

