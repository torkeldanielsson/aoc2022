use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut elevation_map = std::collections::HashMap::new();
    let mut start = glam::ivec2(-10, -10);

    let mut a_positions = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut c = c;
            if c == 'S' {
                c = 'a';
            }
            if c == 'E' {
                start = glam::ivec2(x as i32, y as i32);
                c = 'z';
            }
            if c == 'a' {
                a_positions.push(glam::ivec2(x as i32, y as i32));
            }

            elevation_map.insert(glam::ivec2(x as i32, y as i32), c as i32 - 'a' as i32 + 1);
        }
    }

    let mut open_set = Vec::new();
    open_set.push(start);

    let mut g_score = std::collections::HashMap::new();
    g_score.insert(start, 0);

    while !open_set.is_empty() {
        let p = open_set.pop().unwrap();
        for dir in [
            glam::ivec2(-1, 0),
            glam::ivec2(0, -1),
            glam::ivec2(1, 0),
            glam::ivec2(0, 1),
        ] {
            let neighbor = p + dir;
            if elevation_map.contains_key(&neighbor)
                && elevation_map[&p] <= elevation_map[&neighbor] + 1
            {
                let tentative_score = g_score[&p] + 1;
                if tentative_score < *g_score.get(&neighbor).unwrap_or(&std::i32::MAX) {
                    g_score.insert(neighbor, tentative_score);
                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                    }
                }
            }
        }
    }

    let mut res = std::i32::MAX;

    for ap in a_positions {
        if g_score.contains_key(&ap) && g_score[&ap] < res {
            res = g_score[&ap];
        }
    }

    println!("{res}");

    Ok(())
}
