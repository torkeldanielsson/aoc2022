use std::{error::Error, fs};

use glam::ivec3;

fn fill(
    map: &std::collections::HashSet<glam::IVec3>,
    test_set: &mut std::collections::HashSet<glam::IVec3>,
    pos: &glam::IVec3,
    min: &glam::IVec3,
    max: &glam::IVec3,
) -> bool {
    for dir in [
        ivec3(1, 0, 0),
        ivec3(0, 1, 0),
        ivec3(0, 0, 1),
        ivec3(-1, 0, 0),
        ivec3(0, -1, 0),
        ivec3(0, 0, -1),
    ] {
        let test_pos = *pos + dir;
        if test_pos.x < min.x
            || test_pos.y < min.y
            || test_pos.z < min.z
            || test_pos.x > max.x
            || test_pos.y > max.y
            || test_pos.z > max.z
        {
            return false;
        }
        if !map.contains(&test_pos) && !test_set.contains(&test_pos) {
            test_set.insert(test_pos);
            if !fill(map, test_set, &test_pos, min, max) {
                return false;
            }
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = fs::read_to_string("input")?
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

    let mut min = ivec3(std::i32::MAX, std::i32::MAX, std::i32::MAX);
    let mut max = ivec3(std::i32::MIN, std::i32::MIN, std::i32::MIN);
    for p in &map {
        min.x = std::cmp::min(min.x, p.x);
        min.y = std::cmp::min(min.y, p.y);
        min.z = std::cmp::min(min.z, p.z);
        max.x = std::cmp::max(max.x, p.x);
        max.y = std::cmp::max(max.y, p.y);
        max.z = std::cmp::max(max.z, p.z);
    }

    for x in min.x + 1..max.x {
        for y in min.y + 1..max.y {
            for z in min.z + 1..max.z {
                let test_pos = ivec3(x, y, z);
                if !map.contains(&test_pos) {
                    let mut test_fill = std::collections::HashSet::new();
                    test_fill.insert(test_pos);
                    if fill(&map, &mut test_fill, &test_pos, &min, &max) {
                        map.extend(&test_fill);
                    }
                }
            }
        }
    }

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
