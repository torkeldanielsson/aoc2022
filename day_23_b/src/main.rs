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

    let mut res = 0;

    loop {
        res += 1;
        let mut someone_moved = false;

        if res % 1000 == 0 {
            println!("{res}");
        }

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
                        someone_moved = true;
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

        if !someone_moved {
            break;
        }
    }

    println!("{res}");

    Ok(())
}
