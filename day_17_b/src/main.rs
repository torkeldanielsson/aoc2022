use std::{error::Error, fs};

use glam::ivec2;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum PieceType {
    Flat,
    Plus,
    Angle,
    Stand,
    Ball,
}

fn main() -> Result<(), Box<dyn Error>> {
    let directions = fs::read_to_string("input")?
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("?"),
        })
        .collect::<Vec<_>>();
    let dir_len = directions.len();

    let piece_types = [
        PieceType::Flat,
        PieceType::Plus,
        PieceType::Angle,
        PieceType::Stand,
        PieceType::Ball,
    ];

    let mut piece_shapes: std::collections::HashMap<PieceType, Vec<glam::IVec2>> =
        std::collections::HashMap::new();
    piece_shapes.insert(
        PieceType::Flat,
        vec![ivec2(0, 0), ivec2(1, 0), ivec2(2, 0), ivec2(3, 0)],
    );
    piece_shapes.insert(
        PieceType::Plus,
        vec![
            ivec2(0, 1),
            ivec2(1, 1),
            ivec2(2, 1),
            ivec2(1, 0),
            ivec2(1, 2),
        ],
    );
    piece_shapes.insert(
        PieceType::Angle,
        vec![
            ivec2(0, 0),
            ivec2(1, 0),
            ivec2(2, 0),
            ivec2(2, 1),
            ivec2(2, 2),
        ],
    );
    piece_shapes.insert(
        PieceType::Stand,
        vec![ivec2(0, 0), ivec2(0, 1), ivec2(0, 2), ivec2(0, 3)],
    );
    piece_shapes.insert(
        PieceType::Ball,
        vec![ivec2(0, 0), ivec2(0, 1), ivec2(1, 0), ivec2(1, 1)],
    );

    let mut tallest = 0;

    let mut map = std::collections::HashSet::new();

    for x in 0..7 {
        map.insert(ivec2(x, 0));
    }

    let mut t = 0;

    let haystack_size = 50_000;
    let mut haystack = Vec::new();
    let mut needle = Vec::new();

    let mut previous_tallest = 0;

    let mut repeat = 0;
    let mut height_at_100k = 0;
    let mut deltas_at_repeat_after_100k = Vec::new();

    for i in 0..1000000000000 {
        let delta = tallest - previous_tallest;
        if i < haystack_size {
            haystack.push(delta);
        } else if i < haystack_size + 1000 {
            needle.push(delta);
        } else if i == haystack_size + 1000 {
            let mut last_pos = 0;
            for window in haystack.windows(needle.len()).enumerate() {
                if window.1 == needle {
                    //  println!("{} {}", window.0, window.0 - last_pos);
                    repeat = window.0 - last_pos;
                    last_pos = window.0;
                }
            }
        }
        previous_tallest = tallest;

        if i == 100_000 {
            height_at_100k = tallest as usize;
        }
        if i >= 100_000 && i < 100_000 + repeat {
            deltas_at_repeat_after_100k.push(delta as usize);
        }
        if i > 100_000 + repeat {
            break;
        }

        let piece_type = &piece_types[i % 5];

        let shape = match piece_type {
            PieceType::Flat => &piece_shapes[&PieceType::Flat],
            PieceType::Plus => &piece_shapes[&PieceType::Plus],
            PieceType::Angle => &piece_shapes[&PieceType::Angle],
            PieceType::Stand => &piece_shapes[&PieceType::Stand],
            PieceType::Ball => &piece_shapes[&PieceType::Ball],
        };

        let mut pos = ivec2(2, tallest + 4);

        loop {
            {
                let direction = &directions[t % dir_len];

                let x_before = pos.x;

                pos.x = match direction {
                    Direction::Left => pos.x - 1,
                    Direction::Right => pos.x + 1,
                };
                pos.x = std::cmp::max(pos.x, 0);

                let max_x = match piece_type {
                    PieceType::Flat => 3,
                    PieceType::Plus => 4,
                    PieceType::Angle => 4,
                    PieceType::Stand => 6,
                    PieceType::Ball => 5,
                };
                pos.x = std::cmp::min(max_x, pos.x);

                let mut collission = false;
                for delta in shape {
                    let test_pos = pos + *delta;
                    if map.contains(&test_pos) {
                        collission = true;
                        break;
                    }
                }
                if collission {
                    pos.x = x_before;
                }
            }

            let mut collission = false;
            {
                let y_before = pos.y;

                t += 1;
                pos.y -= 1;

                for delta in shape {
                    let test_pos = pos + *delta;
                    if map.contains(&test_pos) {
                        collission = true;
                        break;
                    }
                }
                if collission {
                    pos.y = y_before;
                    for delta in shape {
                        let add_pos = pos + *delta;
                        map.insert(add_pos);
                    }
                    tallest = std::cmp::max(
                        tallest,
                        pos.y
                            + match piece_type {
                                PieceType::Flat => 0,
                                PieceType::Plus => 2,
                                PieceType::Angle => 2,
                                PieceType::Stand => 3,
                                PieceType::Ball => 1,
                            },
                    );
                }
            }

            if collission {
                break;
            }
        }
    }

    //  println!("{repeat} {height_at_100k} {deltas_at_repeat_after_100k:?}");

    let delta_for_repeats: usize = deltas_at_repeat_after_100k.iter().sum();
    let repeats: usize = (1_000_000_000_000 - 100_000) / repeat - 3;
    let mut full_height = height_at_100k + repeats * delta_for_repeats;
    let leftover_moves = 1_000_000_000_000 - 100_000 - repeats * repeat + 1;

    for i in 0..leftover_moves {
        full_height += deltas_at_repeat_after_100k[i % repeat];
    }

    println!("{full_height}");
    // 1567723342928 too low

    Ok(())
}
