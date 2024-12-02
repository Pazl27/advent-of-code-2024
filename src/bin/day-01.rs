use std::cmp::{max, min};
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let (left_side, right_side) = match read_file() {
        Ok((left, right)) => (left, right),
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    let mut left_side = left_side;
    let mut right_side = right_side;

    let total = simularity_score(&left_side, &right_side)?;
    println!("Total for part 2: {}", total);

    left_side.sort();
    right_side.sort();

    let total = count_total(&left_side, &right_side)?;
    println!("Total for part 1: {}", total);

    Ok(())
}

fn count_total(left_side: &Vec<i32>, right_side: &Vec<i32>) -> Result<i32, Error> {
    if left_side.len() != right_side.len() {
        return Err(Error::new(InvalidData, "Data is not equal"));
    }

    let total: i32 = left_side
        .iter()
        .zip(right_side.iter())
        .map(|(&left, &right)| max(left, right) - min(left, right))
        .sum();

    Ok(total)
}

fn simularity_score(left_side: &Vec<i32>, right_side: &Vec<i32>) -> Result<i32, Error> {
    if left_side.len() != right_side.len() {
        return Err(Error::new(InvalidData, "Data is not equal"));
    }

    let total = left_side
        .iter()
        .map(|&left| {
            let count = right_side.iter().filter(|&&right| right == left).count();
            count as i32 * left
        })
        .sum();

    Ok(total)
}

fn read_file() -> Result<(Vec<i32>, Vec<i32>), Error> {
    let file = File::open("resources/1.txt")?;
    let reader = BufReader::new(file);
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("   ").collect();
        left_side.push(parts[0].parse::<i32>().unwrap());
        right_side.push(parts[1].parse::<i32>().unwrap());
    }
    Ok((left_side, right_side))
}
