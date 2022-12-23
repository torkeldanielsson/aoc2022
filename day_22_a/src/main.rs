use std::{collections::HashMap, error::Error, fs};

use glam::ivec2;

#[derive(PartialEq)]
enum Pos {
    Open,
    Wall,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut max = ivec2(0, 0);

    let map = {
        let mut res = HashMap::new();
        for (y, line) in map.lines().enumerate() {
            max.y = std::cmp::max(max.y, y as i32 + 1);
            for (x, c) in line.chars().enumerate() {
                max.x = std::cmp::max(max.x, x as i32 + 1);
                match c {
                    ' ' => {}
                    '.' => {
                        res.insert(ivec2(x as i32 + 1, y as i32 + 1), Pos::Open);
                    }
                    '#' => {
                        res.insert(ivec2(x as i32 + 1, y as i32 + 1), Pos::Wall);
                    }
                    _ => panic!(),
                }
            }
        }
        res
    };

    let mut pos = ivec2(1, 1);
    let mut dir = ivec2(1, 0);

    while !map.contains_key(&pos) {
        pos += dir;
    }

    let mut path = HashMap::new();

    let mut is_num = true;
    for (i,m) in moves
        .replace('R', " R ")
        .replace('L', " L ")
        .split_ascii_whitespace().enumerate()
    {
        if is_num {
            let num = m.parse::<i32>().unwrap();
            for _ in 0..num {
                let mut test_pos = pos + dir;
                if !map.contains_key(&test_pos) {
                    match (dir.x, dir.y) {
                        (1, 0) => {
                            test_pos.x = 1;
                        }
                        (0, -1) => {
                            test_pos.y = max.y;
                        }
                        (-1, 0) => {
                            test_pos.x = max.x;
                        }
                        (0, 1) => {
                            test_pos.y = 1;
                        }
                        _ => panic!(),
                    }
                    while !map.contains_key(&test_pos) {
                        test_pos += dir;
                    }
                }
                pos = match map.get(&test_pos).unwrap() {
                    Pos::Wall => pos,
                    Pos::Open => test_pos,
                };

                path.insert(
                    pos,
                    match (dir.x, dir.y) {
                        (1, 0) => '>',
                        (0, 1) => 'v',
                        (-1, 0) => '<',
                        (0, -1) => '^',
                        _ => panic!(),
                    },
                );
            }
        } else {
            dir = match (m, dir.x, dir.y) {
                ("L", 1, 0) => ivec2(0, -1),
                ("L", 0, 1) => ivec2(1, 0),
                ("L", -1, 0) => ivec2(0, 1),
                ("L", 0, -1) => ivec2(-1, 0),
                ("R", 1, 0) => ivec2(0, 1),
                ("R", 0, 1) => ivec2(-1, 0),
                ("R", -1, 0) => ivec2(0, -1),
                ("R", 0, -1) => ivec2(1, 0),
                _ => panic!(),
            };
        }
        is_num = !is_num;

        path.insert(
            pos,
            match (dir.x, dir.y) {
                (1, 0) => '>',
                (0, 1) => 'v',
                (-1, 0) => '<',
                (0, -1) => '^',
                _ => panic!(),
            },
        );

        println!("{} {}",pos.x,pos.y);
    }

    for y in 1..=max.y {
        for x in 1..max.x {
            let pos = ivec2(x, y);
            if path.contains_key(&pos) {
                print!("{}", path[&pos]);
            } else if map.contains_key(&pos) {
                print!(
                    "{}",
                    match map[&pos] {
                        Pos::Open => '.',
                        Pos::Wall => '#',
                    }
                );
            } else {
                print!(" ");
            }
        }
        println!();
    }

    let facing_score = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!(),
    };

    println!("{pos} {dir}");

    println!("{}", 1000 * pos.y + 4 * pos.x + facing_score);
    // 104334 too low

    Ok(())
}
