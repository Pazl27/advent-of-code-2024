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
