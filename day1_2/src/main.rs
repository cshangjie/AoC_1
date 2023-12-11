use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn find_first_last_digit_or_substring<'a>(
    input_line: &'a str,
    val_hashmap: &'a HashMap<&str, &str>,
    replacement_hashmap: &'a HashMap<&str, &str>,
    pattern: &Regex,
) -> (Option<i32>, Option<i32>) {
    let modified_line = replacement_hashmap
        .iter()
        .fold(input_line.to_string(), |acc, (key, value)| {
            acc.replace(key, value)
        });

    let matches: VecDeque<&str> = pattern
        .find_iter(&modified_line)
        .map(|mat| mat.as_str())
        .collect();

    let first = matches.front().map(|&s| {
        let value = *val_hashmap.get(s).unwrap_or(&s);
        // println!("First: {}", value);
        value.parse().unwrap()
    });

    let last = matches.back().map(|&s| {
        let value = *val_hashmap.get(s).unwrap_or(&s);
        // println!("Last: {}", value);
        value.parse().unwrap()
    });
    (first, last)
}

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let pattern = Regex::new(r#"[1-9]|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    let replacement_hashmap: HashMap<&str, &str> = [
        ("oneight", "oneeight"),
        ("twone", "twoone"),
        ("threeight", "threeeight"),
        ("fiveight", "fiveeight"),
        ("sevenine", "sevennine"),
        ("eightwo", "eighttwo"),
        ("eighthree", "eightthree"),
        ("nineight", "nineeight"),
    ]
    .iter()
    .cloned()
    .collect();
    let value_hashmap: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize running total
    let mut total: i32 = 0;

    // Process each line in the file
    for line in reader.lines() {
        let input_str = line?;

        let (first, last) = find_first_last_digit_or_substring(
            &input_str,
            &value_hashmap,
            &replacement_hashmap,
            &pattern,
        );

        if let (Some(first), Some(last)) = (first, last) {
            let result_num: i32 = format!("{}{}", first, last).parse().unwrap_or_else(|_| {
                println!("Failed to parse result: {}{}", first, last);
                0
            });
            total += result_num;
        } else if let Some(first) = first {
            let result_num: i32 = format!("{}", first).parse().unwrap_or_else(|_| {
                println!("Failed to parse result: {}", first);
                0
            });
            total += result_num;
        } else if let Some(last) = last {
            let result_num: i32 = format!("{}", last).parse().unwrap_or_else(|_| {
                println!("Failed to parse result: {}", last);
                0
            });
            total += result_num;
        }
    }

    println!("Running Total: {}", total);

    Ok(())
}
