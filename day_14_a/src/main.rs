use glam::ivec2;
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = HashSet::new();

    for line in input.lines() {
        for w in line
            .split(" -> ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.split_once(',').unwrap())
            .map(|t| ivec2(t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
        {
            let delta = match (w[0], w[1]) {
                (a, b) if a.x == b.x && a.y > b.y => ivec2(0, -1),
                (a, b) if a.x == b.x && a.y < b.y => ivec2(0, 1),
                (a, b) if a.y == b.y && a.x > b.x => ivec2(-1, 0),
                (a, b) if a.y == b.y && a.x < b.x => ivec2(1, 0),
                _ => panic!("?"),
            };
            let mut p = w[0];
            map.insert(p);
            while p != w[1] {
                p += delta;
                map.insert(p);
            }
        }
    }

    let count_before_sand = map.len();
    let mut lowest_point = 0;
    for p in &map {
        lowest_point = std::cmp::max(lowest_point, p.y);
    }

    'outer: loop {
        let mut sand = ivec2(500, 0);

        'inner: loop {
            if !map.contains(&(sand + ivec2(0, 1))) {
                sand += ivec2(0, 1);
                if sand.y > lowest_point {
                    break 'outer;
                }
            } else if !map.contains(&(sand + ivec2(-1, 1))) {
                sand += ivec2(-1, 1);
            } else if !map.contains(&(sand + ivec2(1, 1))) {
                sand += ivec2(1, 1);
            } else {
                map.insert(sand);
                break 'inner;
            }
        }
    }

    println!("{}", map.len() - count_before_sand);

    Ok(())
}
