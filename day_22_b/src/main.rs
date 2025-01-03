use std::{collections::HashMap, error::Error, fs, time::Instant};

use glam::ivec2;

#[derive(PartialEq)]
enum Pos {
    Open,
    Wall,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

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
                        res.insert(ivec2(x as i32, y as i32), Pos::Open);
                    }
                    '#' => {
                        res.insert(ivec2(x as i32, y as i32), Pos::Wall);
                    }
                    _ => panic!(),
                }
            }
        }
        res
    };

    let mut replacements = HashMap::new();

    {
        let xa = 50;
        let ya = 0;
        let xb = 0;
        let yb = 150;

        for v in 0..50 {
            replacements.insert(ivec2(xa + v, ya - 1), (ivec2(xb, yb + v), ivec2(1, 0)));
            replacements.insert(ivec2(xb - 1, yb + v), (ivec2(xa + v, ya), ivec2(0, 1)));
        }
    }

    {
        let xa = 50;
        let ya = 50;
        let xb = 0;
        let yb = 100;

        for v in 0..50 {
            replacements.insert(ivec2(xa - 1, ya + v), (ivec2(xb + v, yb), ivec2(0, 1)));
            replacements.insert(ivec2(xb + v, yb - 1), (ivec2(xa, ya + v), ivec2(1, 0)));
        }
    }

    {
        let xa = 50;
        let ya = 49;
        let xb = 0;
        let yb = 100;

        for v in 0..50 {
            replacements.insert(ivec2(xa - 1, ya - v), (ivec2(xb, yb + v), ivec2(1, 0)));
            replacements.insert(ivec2(xb - 1, yb + v), (ivec2(xa, ya - v), ivec2(1, 0)));
        }
    }

    {
        let xa = 99;
        let ya = 50;
        let xb = 100;
        let yb = 49;

        for v in 0..50 {
            replacements.insert(ivec2(xa + 1, ya + v), (ivec2(xb + v, yb), ivec2(0, -1)));
            replacements.insert(ivec2(xb + v, yb + 1), (ivec2(xa, ya + v), ivec2(-1, 0)));
        }
    }

    {
        let xa = 149;
        let ya = 0;
        let xb = 99;
        let yb = 149;

        for v in 0..50 {
            replacements.insert(ivec2(xa + 1, ya + v), (ivec2(xb, yb - v), ivec2(-1, 0)));
            replacements.insert(ivec2(xb + 1, yb - v), (ivec2(xa, ya + v), ivec2(-1, 0)));
        }
    }

    {
        let xa = 50;
        let ya = 149;
        let xb = 49;
        let yb = 150;

        for v in 0..50 {
            replacements.insert(ivec2(xa + v, ya + 1), (ivec2(xb, yb + v), ivec2(-1, 0)));
            replacements.insert(ivec2(xb + 1, yb + v), (ivec2(xa + v, ya), ivec2(0, -1)));
        }
    }

    {
        let xa = 100;
        let ya = 0;
        let xb = 0;
        let yb = 199;

        for v in 0..50 {
            replacements.insert(ivec2(xa + v, ya - 1), (ivec2(xb + v, yb), ivec2(0, -1)));
            replacements.insert(ivec2(xb + v, yb + 1), (ivec2(xa + v, ya), ivec2(0, 1)));
        }
    }

    let mut pos = ivec2(0, 0);
    let mut dir = ivec2(1, 0);

    while !map.contains_key(&pos) {
        pos += dir;
    }

    let mut path = HashMap::new();

    let mut is_num = true;
    for m in moves
        .replace('R', " R ")
        .replace('L', " L ")
        .split_ascii_whitespace()
    {
        if is_num {
            let num = m.parse::<i32>().unwrap();
            for _ in 0..num {
                let mut test_pos = pos + dir;
                let mut test_dir = dir;
                if !map.contains_key(&test_pos) {
                    (test_pos, test_dir) = replacements[&test_pos];
                }
                (pos, dir) = match map.get(&test_pos).unwrap() {
                    Pos::Wall => (pos, dir),
                    Pos::Open => (test_pos, test_dir),
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
    }

    // for y in 1..=max.y {
    //     for x in 1..max.x {
    //         let pos = ivec2(x, y);
    //         if path.contains_key(&pos) {
    //             print!("{}", path[&pos]);
    //         } else if map.contains_key(&pos) {
    //             print!(
    //                 "{}",
    //                 match map[&pos] {
    //                     Pos::Open => '.',
    //                     Pos::Wall => '#',
    //                 }
    //             );
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }

    let facing_score = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!(),
    };

    println!(
        "res: {}, {} us",
        1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing_score,
        t.elapsed().as_micros()
    );

    // 44319 too high

    Ok(())
}
