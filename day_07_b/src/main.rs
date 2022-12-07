use std::collections::HashMap;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut path = Vec::new();
    let mut sizes = HashMap::new();

    for line in input.lines() {
        if line.starts_with("dir") || line.starts_with("$ ls") {
            // ignore
        } else if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd") {
            let split = line.split_at(5);
            path.push(split.1.to_owned());
        } else {
            let split = line.split_once(' ').unwrap();
            let val = split.0.parse::<i32>()?;
            for i in 0..path.len() {
                sizes
                    .entry(path[..=i].join("/"))
                    .and_modify(|sum| *sum += val)
                    .or_insert(val);
            }
        }
    }

    let outer_size = sizes["/"];
    let min_space_to_free = outer_size - 40000000;

    let mut res = 70000000;

    for size in sizes {
        if size.1 >= min_space_to_free && size.1 < res {
            res = size.1;
        }
    }

    println!("res: {res}");

    Ok(())
}
