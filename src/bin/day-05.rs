use std::io::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<()> {
    let file = File::open("resources/5.txt")?;
    let reader = BufReader::new(file);
    let mut line_break = false;

    let mut map: HashMap<i32,i32> = HashMap::new();
    let mut vec: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line_break {
            if let Some(_) = line.find("|") {
                let temp = line.split("|").collect::<Vec<&str>>();
                let key = temp[0].trim().parse::<i32>().unwrap();
                let value = temp[1].trim().parse::<i32>().unwrap();
                map.insert(key, value);
            }
            if line.is_empty() {
                line_break = true;
            }
        } else {
            let row: Vec<i32> = line.split(",")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            vec.push(row);
        }
    }

    let total = part1(&map, &vec)?;
    println!("Total of Part 1: {}", total);

    Ok(())
}

fn part1(map: &HashMap<i32,i32>, vec: &Vec<Vec<i32>>) -> Result<i32> {
    let mut total = 0;

    for row in vec {
        let mut right_order = true;
        for i in 0..row.len() {
            if let Some(&value) = map.get(&(row[i])) {
                for j in 0..row.len() {
                    if row[j] == value {
                        if i > j {
                            right_order = false;  
                            break;
                        } 
                    }
                }
            }
        }
        if right_order {
            total += row.get(row.len()/2).unwrap();
        }
    }
    Ok(total)
}


#[cfg(test)]

#[test]
fn test_part1() {
    let mut map: HashMap<i32,i32> = HashMap::new();
    map.insert(47, 53);
    map.insert(97, 13);
    map.insert(97, 61);
    map.insert(97, 47);
    map.insert(75, 29);
    map.insert(61, 13);
    map.insert(75, 53);
    map.insert(29, 13);
    map.insert(97, 29);
    map.insert(53, 29);
    map.insert(61, 53);
    map.insert(97, 53);
    map.insert(61, 29);
    map.insert(47, 13);
    map.insert(75, 47);
    map.insert(97, 75);
    map.insert(47, 61);
    map.insert(75, 61);
    map.insert(47, 29);
    map.insert(75, 13);
    map.insert(53, 13);

    let vec: Vec<Vec<i32>> = vec![
        vec![75,47,61,53,29],
        vec![97,61,53,29,13],
        vec![75,29,13],
        vec![75,97,47,61,53],
        vec![61,13,29],
        vec![97,13,75,29,47]
    ];

    assert_eq!(part1(&map, &vec).unwrap(), 143);
}
