use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let mut total_1 = 0;
    let mut total_2 = 0;
    let file = File::open("resources/7.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let (left, right) = line.split_at(line.find(":").unwrap());

        let target: i64 = left.trim().parse().unwrap();
        let arr: Vec<i64> = right
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();

        if part1(target, &arr) {
            total_1 += target;
        }

        if part2(target, &arr) {
            total_2 += target;
        }
    }

    println!("Total for part 1: {}", total_1);
    println!("Total for part 2: {}", total_2);

    Ok(())
}

fn part1(target: i64, arr: &[i64]) -> bool {
    if arr.is_empty() {
        return false;
    }
    if arr.len() == 1 {
        return arr[0] == target;
    }
    if target % arr[arr.len() - 1] == 0
        && part1(target / arr[arr.len() - 1], &arr[..arr.len() - 1])
    {
        return true;
    }
    if target > arr[arr.len() - 1]
        && part1(target - arr[arr.len() - 1], &arr[..arr.len() - 1])
    {
        return true;
    }

    false
}

fn part2(target: i64, arr: &[i64]) -> bool {
    if arr.is_empty() {
        return false;
    }
    if arr.len() == 1 {
        return arr[0] == target;
    }
    if target % arr[arr.len() - 1] == 0
        && part2(target / arr[arr.len() - 1], &arr[..arr.len() - 1])
    {
        return true;
    }
    if target > arr[arr.len() - 1]
        && part2(target - arr[arr.len() - 1], &arr[..arr.len() - 1])
    {
        return true;
    }
    
    let s_target = target.to_string();
    let s_last = arr[arr.len() - 1].to_string();

    if s_target.len() > s_last.len() && s_target.ends_with(&s_last) {
        let new_target = s_target[..s_target.len() - s_last.len()].parse::<i64>().unwrap();
        if part2(new_target, &arr[..arr.len() - 1].to_vec()) {
            return true;
        }
    }
    false
}
