use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

use glam::{ivec2, IVec2};

fn parse_map(input: &str) -> HashSet<IVec2> {
    let mut map = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(ivec2(x as i32, y as i32));
            }
        }
    }

    map
}

fn print_map(map: &HashSet<IVec2>) {
    for y in 0..12 {
        for x in 0..14 {
            if map.contains(&ivec2(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = parse_map(&fs::read_to_string("input")?);

    let neighbor_deltas = [
        ivec2(-1, 0),
        ivec2(-1, -1),
        ivec2(0, -1),
        ivec2(1, -1),
        ivec2(1, 0),
        ivec2(1, 1),
        ivec2(0, 1),
        ivec2(-1, 1),
    ];

    let proposals_deltas = [
        [ivec2(-1, -1), ivec2(0, -1), ivec2(1, -1)],
        [ivec2(-1, 1), ivec2(0, 1), ivec2(1, 1)],
        [ivec2(-1, -1), ivec2(-1, 0), ivec2(-1, 1)],
        [ivec2(1, -1), ivec2(1, 0), ivec2(1, 1)],
    ];
    let mut proposal_start = 0;

    for _ in 0..10 {
        let mut proposals = HashMap::new();

        for e in &map {
            let mut should_move = false;
            for nd in &neighbor_deltas {
                if map.contains(&(*e + *nd)) {
                    should_move = true;
                    break;
                }
            }

            if should_move {
                'outer: for i in 0..4 {
                    let deltas = &proposals_deltas[(proposal_start + i) % 4];
                    for delta in deltas {
                        if map.contains(&(*e + *delta)) {
                            continue 'outer;
                        }
                    }
                    let proposal_pos = *e + deltas[1];
                    let new_val = proposals.get(&proposal_pos).unwrap_or(&0) + 1;
                    proposals.insert(proposal_pos, new_val);
                    break 'outer;
                }
            }
        }

        let mut new_map = HashSet::new();

        for e in &map {
            let mut should_move = false;
            for nd in &neighbor_deltas {
                if map.contains(&(*e + *nd)) {
                    should_move = true;
                    break;
                }
            }

            if should_move {
                let mut moved = false;
                'outer: for i in 0..4 {
                    let deltas = &proposals_deltas[(proposal_start + i) % 4];
                    for delta in deltas {
                        if map.contains(&(*e + *delta)) {
                            continue 'outer;
                        }
                    }
                    let proposal_pos = *e + deltas[1];
                    if proposals[&proposal_pos] == 1 {
                        new_map.insert(proposal_pos);
                        moved = true;
                    }
                    break 'outer;
                }
                if !moved {
                    new_map.insert(*e);
                }
            } else {
                new_map.insert(*e);
            }
        }

        map = new_map;

        proposal_start = (proposal_start + 1) % 4;
    }

    let mut min = ivec2(std::i32::MAX, std::i32::MAX);
    let mut max = ivec2(std::i32::MIN, std::i32::MIN);
    for e in &map {
        min.x = std::cmp::min(min.x, e.x);
        min.y = std::cmp::min(min.y, e.y);
        max.x = std::cmp::max(max.x, e.x);
        max.y = std::cmp::max(max.y, e.y);
    }

    let mut res = 0;
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if !map.contains(&ivec2(x, y)) {
                res += 1;
            }
        }
    }

    println!("{res}");

    Ok(())
}
