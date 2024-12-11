use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("resources/5.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row = line.chars().collect();
        matrix.push(row);
    }

    let total = part1(&matrix)?;
    println!("Total of Part 1: {}", total);

    Ok(())
}

fn part1(matrix: &Vec<Vec<char>>) -> Result<i32> {
    let mut total = 0;

    find_and_move(&matrix);

    Ok(total)
}

fn find_and_move(matrix: &Vec<Vec<char>>) {
    let mut x = 0;
    let mut y = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '<' || matrix[i][j] == '>' || matrix[i][j] == '^' || matrix[i][j] == 'v' {
                x = i;
                y = j;
                
                // do move logic here and then call this methode again recursively until character
                // is not found
            }
        }
    }
}
