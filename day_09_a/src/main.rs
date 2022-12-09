use std::collections::HashSet;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = HashSet::new();
    map.insert((0, 0));

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);

    for (dir, dist) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let dir = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("?"),
        };
        for _ in 0..dist.parse::<i32>().unwrap() {
            h.0 += dir.0;
            h.1 += dir.1;

            if (h.0 - t.0).abs() > 1 {
                t.1 = h.1;
                t.0 += (h.0 - t.0) / 2;
            }
            if (h.1 - t.1).abs() > 1 {
                t.0 = h.0;
                t.1 += (h.1 - t.1) / 2;
            }
            map.insert(t);
        }
    }

    println!("{}", map.len());

    Ok(())
}
