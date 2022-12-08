use std::collections::HashMap;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let (mut x_max, mut y_max) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let d = c.to_string().parse::<i32>().unwrap();
            map.insert((x as i32, y as i32), d);
            x_max = std::cmp::max(x as i32, x_max as i32);
            y_max = std::cmp::max(y as i32, y_max as i32);
        }
    }

    let mut best_score = 0;

    for x_c in 0..=x_max {
        for y_c in 0..=y_max {
            let mut score = 1;

            for dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut acc = 0;

                let mut pos = (x_c, y_c);
                let height = map[&pos];

                loop {
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    if map.contains_key(&pos) {
                        let tree = map[&pos];
                        if tree < height {
                            acc += 1;
                        } else {
                            acc += 1;
                            break;
                        }
                    } else {
                        break;
                    }
                }

                score *= acc;
            }

            best_score = std::cmp::max(best_score, score);
        }
    }

    println!("best_score: {best_score}");

    Ok(())
}
