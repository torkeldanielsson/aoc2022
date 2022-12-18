use std::{error::Error, fs};

use glam::ivec3;

fn main() -> Result<(), Box<dyn Error>> {
    let map = fs::read_to_string("input")?
        .lines()
        .map(|s| {
            let parts = s.split(',').collect::<Vec<_>>();
            ivec3(
                parts[0].to_string().parse::<i32>().unwrap(),
                parts[1].to_string().parse::<i32>().unwrap(),
                parts[2].to_string().parse::<i32>().unwrap(),
            )
        })
        .collect::<std::collections::HashSet<_>>();

    let mut surface_area = 0;

    for p in &map {
        for dir in [
            ivec3(1, 0, 0),
            ivec3(0, 1, 0),
            ivec3(0, 0, 1),
            ivec3(-1, 0, 0),
            ivec3(0, -1, 0),
            ivec3(0, 0, -1),
        ] {
            if !map.contains(&(*p + dir)) {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");

    Ok(())
}
