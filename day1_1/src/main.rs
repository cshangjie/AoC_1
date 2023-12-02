use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn extract_and_sum_numbers(line: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let digits: Vec<char> = re.find_iter(line)
    .flat_map(|num| num.as_str().chars())
    .collect();
    let first_digit = digits[0].to_digit(10).unwrap_or(0);
    let last_digit = digits[digits.len() - 1].to_digit(10).unwrap_or(0);
    let two_digit_number = first_digit * 10 + last_digit;
    two_digit_number as i32
}

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let total_sum: i32 = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| extract_and_sum_numbers(&line))
        .sum();

    println!("Total Sum: {}", total_sum);

    Ok(())
}
