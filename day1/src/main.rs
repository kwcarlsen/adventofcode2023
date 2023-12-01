use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_digit(line: &str) -> Option<i32> {
    for c in line.chars() {
        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap() as i32);
        }
    }
    None
}

fn find_last_digit(line: &str) -> Option<i32> {
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap() as i32);
        }
    }
    None
}

fn to_digit(input: &str) -> i32 {
    match input {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("Invalid input {}", input),
    }
}

fn find_digits_or_word(line: &str) -> i32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let first = re.captures(line).unwrap();

    let re = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let second = re.captures(line).unwrap();
    to_digit(&first[1]) * 10 + to_digit(&second[1])
}

fn find_calibration_value(line: &str) -> i32 {
    let first_digit = find_first_digit(line).unwrap();
    let last_digit = find_last_digit(line).unwrap();
    first_digit * 10 + last_digit
}

fn solution(file: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {            
            sum += find_calibration_value(&line);
        }
    }
    sum
}

fn solution2(file: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            sum += find_digits_or_word(&line);
        }
    }
    sum
}

fn main() {
    println!("Example: {}", solution("example.txt"));
    println!("First Answer: {}", solution("input.txt"));
    println!("Second Answer: {}", solution2("input.txt"));

}
