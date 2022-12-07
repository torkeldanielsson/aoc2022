use std::collections::HashMap;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut path = Vec::new();
    let mut sizes = HashMap::new();

    for line in input.lines() {
        if line == "$ cd /" || line.starts_with("dir") || line.starts_with("$ ls") {
            // ignore
        } else if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd") {
            let split = line.split_at(5);
            path.push(split.1.to_owned());
            // println!("{}", path.join("/"));
        } else {
            let split = line.split_once(' ').unwrap();
            let val = split.0.parse::<i32>()?;
            // println!("{val}");
            for i in 0..path.len() {
                sizes
                    .entry(path[..=i].join("/"))
                    .and_modify(|sum| *sum += val)
                    .or_insert(val);
            }
        }
    }

    let mut res = 0;

    for size in sizes {
        if size.1 <= 100000 {
            res += size.1;
        }
    }

    println!("res: {res}");

    Ok(())
}
