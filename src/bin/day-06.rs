use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = File::open("resources/6.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row = line.chars().collect();
        matrix.push(row);
    }

    let total = part1(&mut matrix)?;
    println!("Total of Part 1: {}", total);

    Ok(())
}

fn part1(matrix: &mut Vec<Vec<char>>) -> Result<i32> {
    let mut total = 0;

    let mut start_direction = ' ';
    let mut start_position = (0, 0);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^'
                || matrix[i][j] == 'v'
                || matrix[i][j] == '<'
                || matrix[i][j] == '>'
            {
                start_direction = matrix[i][j];
                start_position = (i, j);
            }
        }
    }

    loop {
        match move_and_mark(matrix, start_direction, &mut start_position) {
            Some(new_dir) => {
                start_direction = new_dir;
            }
            None => {
                break;
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn turn_right(direction: char) -> char {
    match direction {
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        '^' => '>',
        _ => direction,
    }
}

fn move_and_mark(
    matrix: &mut Vec<Vec<char>>,
    direction: char,
    position: &mut (usize, usize),
) -> Option<char> {
    let (mut i, mut j) = *position;
    matrix[i][j] = 'X';
    match direction {
        '^' => {
            while i as i32 - 1 >= 0 {
                if matrix[i - 1][j] == '#' {
                    break;
                }
                i -= 1;
                matrix[i][j] = 'X';
            }
            if i == 0 {
                return None;
            }
        }
        '>' => {
            while j + 1 < matrix[i].len() {
                if matrix[i][j + 1] == '#' {
                    break;
                }
                j += 1;
                matrix[i][j] = 'X';
            }
            if j == matrix[i].len() - 1 {
                return None;
            }
        }
        'v' => {
            while i + 1 < matrix.len() {
                if matrix[i + 1][j] == '#' {
                    break;
                }
                i += 1;
                matrix[i][j] = 'X';
            }
            if i == matrix.len() - 1 {
                return None;
            }
        }
        '<' => {
            while j as i32 - 1 >= 0 {
                if matrix[i][j - 1] == '#' {
                    break;
                }
                j -= 1;
                matrix[i][j] = 'X';
            }
            if j == 0 {
                return None;
            }
        }
        _ => return None,
    }
    *position = (i, j);
    Some(turn_right(direction))
}

#[cfg(test)]
#[test]
fn test_part1() {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    matrix.push("....#.....".chars().collect());
    matrix.push(".........#".chars().collect());
    matrix.push("..........".chars().collect());
    matrix.push("..#.......".chars().collect());
    matrix.push(".......#..".chars().collect());
    matrix.push("..........".chars().collect());
    matrix.push(".#..^.....".chars().collect());
    matrix.push("........#.".chars().collect());
    matrix.push("#.........".chars().collect());
    matrix.push("......#...".chars().collect());

    assert_eq!(part1(&mut matrix).unwrap(), 41);
}
