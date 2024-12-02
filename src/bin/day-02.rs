use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut counter = 0;
    let file = File::open("resources/2.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if part1(&line).expect("Failed to parse line") {
            counter += 1;
        }
    }
    println!("Total: {}", counter);
}

fn part1(line: &str) -> Result<bool, String> {
    let vec: Result<Vec<i32>, _> = line.split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect();

    let vec = vec.map_err(|_| "Failed to parse input".to_string())?;

    let mut only_inc = true;
    let mut only_dec = true;

    for i in 0..vec.len() - 1 {
        let diff = vec[i + 1] - vec[i];

        if !(1 <= diff.abs() && diff.abs() <= 3) {
            return Ok(false);
        }

        if diff > 0 {
            only_dec = false;
        } else if diff < 0 {
            only_inc = false;
        }
    }

    Ok(only_inc || only_dec)
}
