use std::{collections::HashSet, error::Error, fs};

use glam::{ivec2, IVec2};

struct Blizzard {
    pos: IVec2,
    dir: IVec2,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut blizzards = Vec::new();

    for (y, line) in fs::read_to_string("input")?.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {}
                '.' => {}
                '<' => blizzards.push(Blizzard {
                    pos: ivec2(x as i32, y as i32),
                    dir: ivec2(-1, 0),
                }),
                '>' => blizzards.push(Blizzard {
                    pos: ivec2(x as i32, y as i32),
                    dir: ivec2(1, 0),
                }),
                '^' => blizzards.push(Blizzard {
                    pos: ivec2(x as i32, y as i32),
                    dir: ivec2(0, -1),
                }),
                'v' => blizzards.push(Blizzard {
                    pos: ivec2(x as i32, y as i32),
                    dir: ivec2(0, 1),
                }),
                _ => panic!(),
            }
        }
    }

    let min = ivec2(1, 1);
    let mut max = ivec2(0, 0);
    for b in &blizzards {
        max.x = std::cmp::max(max.x, b.pos.x);
        max.y = std::cmp::max(max.y, b.pos.y);
    }

    let mut walk = HashSet::new();
    walk.insert(ivec2(1, 0));

    let mut res = 0;

    loop {
        for b in &mut blizzards {
            b.pos += b.dir;
            if b.pos.x < min.x {
                b.pos.x = max.x
            }
            if b.pos.y < min.y {
                b.pos.y = max.y
            }
            if b.pos.x > max.x {
                b.pos.x = min.x
            }
            if b.pos.y > max.y {
                b.pos.y = min.y
            }
        }

        let mut new_walk = HashSet::new();

        for p in walk {
            'neighbors: for n in [
                ivec2(-1, 0),
                ivec2(0, -1),
                ivec2(1, 0),
                ivec2(0, 1),
                ivec2(0, 0),
            ] {
                let test_pos = p + n;
                for b in &blizzards {
                    if b.pos == test_pos {
                        continue 'neighbors;
                    }
                }
                if test_pos.x >= min.x
                    && test_pos.y >= min.y
                    && test_pos.x <= max.x
                    && test_pos.y <= max.y
                {
                    new_walk.insert(test_pos);
                }
            }
        }

        walk = new_walk;
        walk.insert(ivec2(1, 0));

        res += 1;

        if walk.contains(&max) {
            res += 1;
            break;
        }
    }

    println!("{res}");

    Ok(())
}
