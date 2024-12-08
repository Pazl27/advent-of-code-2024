use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("resources/4.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let total = part1(&matrix)?;
    println!("Part 1: {}", total);

    let total = part2(&matrix)?;
    println!("Part 2: {}", total);

    Ok(())
}

fn part1(matrix: &[Vec<char>]) -> Result<i32> {
    let mut total = 0;

    for row_index in 0..matrix.len() {
        let row = &matrix[row_index];
        for col_index in 0..row.len() {
            if row[col_index] == 'X' {
                // Horizontal check (left to right)
                if col_index + 3 < row.len() {
                    let after_x: String = row[col_index + 1..col_index + 4].iter().collect();
                    if after_x == "MAS" {
                        total += 1;
                    }
                }
                // Horizontal check (right to left)
                if col_index >= 3 {
                    let before_x: String = row[col_index - 3..col_index].iter().collect();
                    if before_x == "SAM" {
                        total += 1;
                    }
                }

                // Vertical check (downward)
                if row_index + 3 < matrix.len() {
                    let below_x: String = matrix[row_index + 1][col_index].to_string()
                        + &matrix[row_index + 2][col_index].to_string()
                        + &matrix[row_index + 3][col_index].to_string();
                    if below_x == "MAS" {
                        total += 1;
                    }
                }
                // Vertical check (upward)
                if row_index >= 3 {
                    let ontop_x: String = matrix[row_index - 3][col_index].to_string()
                        + &matrix[row_index - 2][col_index].to_string()
                        + &matrix[row_index - 1][col_index].to_string();
                    if ontop_x == "SAM" {
                        total += 1;
                    }
                }

                // Diagonal check (top-left to bottom-right)
                if col_index + 3 < row.len() && row_index + 3 < matrix.len() {
                    let diag_x: String = matrix[row_index + 1][col_index + 1].to_string()
                        + &matrix[row_index + 2][col_index + 2].to_string()
                        + &matrix[row_index + 3][col_index + 3].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
                // Diagonal check (bottom-right to top-left)
                if col_index >= 3 && row_index >= 3 {
                    let diag_x: String = matrix[row_index - 1][col_index - 1].to_string()
                        + &matrix[row_index - 2][col_index - 2].to_string()
                        + &matrix[row_index - 3][col_index - 3].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
                // Diagonal check (top-right to bottom-left)
                if col_index >= 3 && row_index + 3 < matrix.len() {
                    let diag_x: String = matrix[row_index + 1][col_index - 1].to_string()
                        + &matrix[row_index + 2][col_index - 2].to_string()
                        + &matrix[row_index + 3][col_index - 3].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
                // Diagonal check (bottom-left to top-right)
                if col_index + 3 < row.len() && row_index >= 3 {
                    let diag_x: String = matrix[row_index - 1][col_index + 1].to_string()
                        + &matrix[row_index - 2][col_index + 2].to_string()
                        + &matrix[row_index - 3][col_index + 3].to_string();
                    if diag_x == "MAS" {
                        total += 1;
                    }
                }
            }
        }
    }

    Ok(total)
}

fn part2(matrix: &Vec<Vec<char>>) -> Result<i32> {
    let pairs = vec![
        (-1, -1), // top-left diagonal
        (-1, 1),  // top-right diagonal
        (1, 1),   // bottom-right diagonal
        (1, -1),  // bottom-left diagonal
    ];
    let mut total = 0;

    for row_index in 0..matrix.len() {
        let row = &matrix[row_index];
        for col_index in 0..row.len() {
            if matrix[row_index][col_index] == 'A' {
                if row_index >= 1 && col_index >= 1 && row_index < matrix.len() - 1 && col_index < row.len() - 1 {
                    let mut word = String::new();
                    for pair in &pairs {
                        let new_row = row_index as i32 + pair.0;
                        let new_col = col_index as i32 + pair.1;
                        if new_row >= 0 && new_row < matrix.len() as i32 && new_col >= 0 && new_col < row.len() as i32 {
                            word.push(matrix[new_row as usize][new_col as usize]);
                        }
                    }
                    if word == "MSSM" || word == "SSMM" || word == "SMMS" {
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
fn test_all_cases() {
    let empty_matrix: Vec<Vec<char>> = Vec::new();
    assert_eq!(part1(&empty_matrix).unwrap(), 0);

    let no_x_matrix = vec![
        vec!['S', 'A', 'M', 'S', 'A'],
        vec!['M', 'A', 'A', 'S', 'M'],
        vec!['A', 'M', 'M', 'A', 'S'],
        vec!['X', 'M', 'A', 'X', 'M'],
    ];
    assert_eq!(part1(&no_x_matrix).unwrap(), 2);

// .M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........
    
    let matrix = vec![
        vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
        vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
        vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
        vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
        vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
        vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(part2(&matrix).unwrap(), 9);
}

