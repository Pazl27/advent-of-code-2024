use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut part1_total = 0;
    let mut part2_total = 0;
    let file = File::open("resources/2.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if part1(&line).expect("Failed to parse line") {
            part1_total += 1;
        }

        if part2(&line).expect("Failed to parse line") {
            part2_total += 1;
        }
    }
    println!("Total for part 1: {}", part1_total);
    println!("Total for part 2: {}", part2_total);
}

fn is_ok(vec: &Vec<i32>) -> bool {
    let mut only_inc = true;
    let mut only_dec = true;

    for i in 0..vec.len() - 1 {
        let diff = vec[i + 1] - vec[i];

        if !(1 <= diff.abs() && diff.abs() <= 3) {
            return false;
        }

        if diff > 0 {
            only_dec = false;
        } else if diff < 0 {
            only_inc = false;
        }
    }

    only_inc || only_dec
}

fn part1(line: &str) -> Result<bool, String> {
    let vec: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|_| "Failed to parse input".to_string())?;

    Ok(is_ok(&vec))
}

fn part2(line: &str) -> Result<bool, String> {
    let vec: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|_| "Failed to parse input".to_string())?;

    if is_ok(&vec) {
        return Ok(true);
    }

    for i in 0..vec.len() {
        let mut vec_clone = vec.clone();
        vec_clone.remove(i);

        if is_ok(&vec_clone) {
            return Ok(true);
        }
    }
    Ok(false)
}
