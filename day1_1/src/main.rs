use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::time::Instant;

fn extract_and_sum_numbers(line: &str, re: &Regex) -> i32 {
    let digits: Vec<char> = re.find_iter(line)
        .flat_map(|num| num.as_str().chars())
        .collect();
    let first_digit = digits.get(0).copied().and_then(|d| d.to_digit(10)).unwrap_or(0);
    let last_digit = digits.last().copied().and_then(|d| d.to_digit(10)).unwrap_or(0);
    let two_digit_number = first_digit * 10 + last_digit;
    two_digit_number as i32
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let file_path = "input.txt";
    let file_open_time = Instant::now();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let file_open_duration = file_open_time.elapsed();

    let re = Regex::new(r"\d+").expect("Invalid regex pattern");

    let lines_processing_time = Instant::now();
    let total_sum: i32 = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| extract_and_sum_numbers(&line, &re))
        .sum();
    let lines_processing_duration = lines_processing_time.elapsed();

    let overall_duration = start_time.elapsed();
    let file2 = File::open(file_path)?;
    let reader2 = io::BufReader::new(file2);
    let num_lines = reader2.lines().count();
    let avg_time_per_line = lines_processing_duration / num_lines as u32;
    println!("Total Sum: {}", total_sum);
    println!("File Open Time: {:?}", file_open_duration);
    println!("Lines Processing Time: {:?}", lines_processing_duration);
    println!("Average Time Per Line: {:?}", avg_time_per_line);
    println!("Overall Run Time: {:?}", overall_duration);

    Ok(())
}
