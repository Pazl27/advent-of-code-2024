use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;


fn main() -> Result<()> {

    let file = File::open("resources/4.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let mut vec: Vec<char> = Vec::new();
        line?.chars().for_each(|c| {
            vec.push(c);
        });
        
        matrix.push(vec);
    }

    let total = part1(&matrix)?;
    println!("Part 1: {}", total);

    Ok(())
}

fn part1(matrix: &Vec<Vec<char>>) -> Result<i32> {
    let mut total = 0;

    for row_index in 0..matrix.len() {
        let row = &matrix[row_index];
        for i in 0..row.len() {
            if row[i] == 'X' {
                if i >= 3 {
                    let before_x: String = row[i - 3..i].iter().collect();
                    if before_x == "SAM" {
                        total += 1;
                    }
                }
                if i + 3 < row.len() {
                    let after_x: String = row[i + 1..i + 4].iter().collect();
                    if after_x == "MAS" {
                        total += 1;
                    }
                }
                if row_index >= 3 {
                    let ontop_x: String = matrix[row_index - 3][i].to_string() + &matrix[row_index - 2][i].to_string() + &matrix[row_index - 1][i].to_string();
                    if ontop_x == "SAM" {
                        total += 1;
                    }
                }
                if row_index + 3 < matrix.len() {
                    let below_x: String = matrix[row_index + 1][i].to_string() + &matrix[row_index + 2][i].to_string() + &matrix[row_index + 3][i].to_string();
                    if below_x == "MAS" {
                        total += 1;
                    }
                }
                if i >= 3 && row_index >= 3 {
                    let diag_x: String = matrix[row_index - 3][i - 3].to_string() + &matrix[row_index - 2][i - 2].to_string() + &matrix[row_index - 1][i - 1].to_string();
                    if diag_x == "SAM" {
                        total += 1;
                    }
                }
                if i + 3 < row.len() && row_index + 3 < matrix.len() {
                    let diag_x: String = matrix[row_index + 1][i + 1].to_string() + &matrix[row_index + 2][i + 2].to_string() + &matrix[row_index + 3][i + 3].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
                if i >= 3 && row_index + 3 < matrix.len() {
                    let diag_x: String = matrix[row_index + 1][i - 3].to_string() + &matrix[row_index + 2][i - 2].to_string() + &matrix[row_index + 3][i - 1].to_string();
                    if diag_x == "SAM" {
                        total += 1;
                    }
                }
                if i + 3 < row.len() && row_index >= 3 {
                    let diag_x: String = matrix[row_index - 3][i + 3].to_string() + &matrix[row_index - 2][i + 2].to_string() + &matrix[row_index - 1][i + 1].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
#[test]
fn simple_input() {
    let matrix = vec![
        vec!['X', 'M', 'A', 'S', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'S', 'A', 'M', 'X', 'X', 'X', 'X', 'M', 'X'],
        vec!['M', 'A', 'S', 'X', 'X', 'X', 'M', 'A', 'X', 'X'],
        vec!['A', 'M', 'S', 'X', 'X', 'X', 'S', 'A', 'X', 'X'],
        vec!['S', 'X', 'M', 'X', 'X', 'X', 'X', 'X', 'S', 'X'],
    ];

    assert_eq!(part1(&matrix).unwrap(), 6);
}
