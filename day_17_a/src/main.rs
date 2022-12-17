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

    for i in 0..2022 {
        let piece_type = &piece_types[i % 5];

        let shape = match piece_type {
            PieceType::Flat => &piece_shapes[&PieceType::Flat],
            PieceType::Plus => &piece_shapes[&PieceType::Plus],
            PieceType::Angle => &piece_shapes[&PieceType::Angle],
            PieceType::Stand => &piece_shapes[&PieceType::Stand],
            PieceType::Ball => &piece_shapes[&PieceType::Ball],
        };

        let mut pos = ivec2(2, tallest + 4);

        // for dy in 0..22 {
        //     let y = pos.y + 4 - dy;
        //     print!("|");
        //     for x in 0..7 {
        //         let mut piece_hit = false;
        //         for delta in shape {
        //             let piece_pos = pos + *delta;
        //             if ivec2(x, y) == piece_pos {
        //                 piece_hit = true;
        //             }
        //         }

        //         if piece_hit {
        //             print!("@")
        //         } else if map.contains(&ivec2(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("|");
        // }
        // println!();

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

                if collission {
                    //  println!("would have collided with wall {direction:?}");
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

                if collission {
                    //   println!("would have collided with piece or ground!");
                }
            }

            //  for dy in 0..8 {
            //      let y = pos.y + 4 - dy;
            //      print!("|");
            //      for x in 0..7 {
            //          let mut piece_hit = false;
            //          for delta in shape {
            //              let piece_pos = pos + *delta;
            //              if ivec2(x, y) == piece_pos {
            //                  piece_hit = true;
            //              }
            //          }
            //          if collission {
            //              piece_hit = false;
            //          }
            //
            //          if piece_hit {
            //              print!("@")
            //          } else if map.contains(&ivec2(x, y)) {
            //              print!("#");
            //          } else {
            //              print!(".");
            //          }
            //      }
            //      println!("|");
            //  }
            //  println!();

            if collission {
                break;
            }
        }
    }

    println!("{tallest}");

    Ok(())
}
