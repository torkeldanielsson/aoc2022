use std::collections::HashSet;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = HashSet::new();
    map.insert((0, 0));

    let rope_len = 10;

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_len];

    for (dir, dist) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let dir = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("?"),
        };
        for _ in 0..dist.parse::<i32>().unwrap() {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for i in 0..(rope_len - 1) {
                let h = rope[i];
                let mut t = &mut rope[i + 1];

                if (h.0 - t.0).abs() > 1 && (h.1 - t.1).abs() > 1 {
                    t.0 += (h.0 - t.0) / 2;
                    t.1 += (h.1 - t.1) / 2;
                } else {
                    if (h.0 - t.0).abs() > 1 {
                        t.1 = h.1;
                        t.0 += (h.0 - t.0) / 2;
                    }
                    if (h.1 - t.1).abs() > 1 {
                        t.0 = h.0;
                        t.1 += (h.1 - t.1) / 2;
                    }
                }
            }

            map.insert(*rope.last().unwrap());
        }
    }

    println!("{}", map.len());

    Ok(())
}
