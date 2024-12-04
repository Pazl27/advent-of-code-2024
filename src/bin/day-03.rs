use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn main() {
    let file = File::open("resources/3.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let text = lines.join("\n");

    let result = part1(&text).unwrap();
    println!("Result from part 1: {}", result);

    let result = part2(&text).unwrap();
    println!("Result from part 2: {}", result);
}

fn part1(text: &str) -> Result<u64, ParseIntError> {
    let mut total = 0;

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to create regex");

    for cap in regex.captures_iter(text) {
        let x: u64 = cap[1].parse()?;
        let y: u64 = cap[2].parse()?;
        total += x * y;
    }

    Ok(total)
}

fn part2(text: &str) -> Result<u64, ParseIntError> {
    let mut total = 0;
    let regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").expect("Failed to create regex");
    let mut check = true;

    for cap in regex.captures_iter(text) {
        if let Some(matched) = cap.get(0) {
            let matched_str = matched.as_str();

            if matched_str.starts_with("mul") {
                let mut iter = matched_str[4..matched_str.len() - 1].split(',');
                let x: u64 = iter.next().unwrap().parse()?;
                let y: u64 = iter.next().unwrap().parse()?;
                if check {
                    total += x * y;
                }
            } else if matched_str.starts_with("don't") {
                check = false;
            } else if matched_str.starts_with("do") {
                check = true;
            }
        }
    }

    Ok(total)
}
