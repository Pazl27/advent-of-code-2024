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

    Ok(())
}
