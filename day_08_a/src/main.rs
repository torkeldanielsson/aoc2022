use std::collections::HashMap;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map: HashMap<(usize, usize), (i32, bool)> = HashMap::new();

    let (mut x_max, mut y_max) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let d = c.to_string().parse::<i32>().unwrap();
            map.insert((x, y), (d, false));
            x_max = std::cmp::max(x, x_max);
            y_max = std::cmp::max(y, y_max);
        }
    }

    for x in 0..=x_max {
        let mut height = -1;
        for y in 0..=y_max {
            let mut tree = map.get_mut(&(x, y)).unwrap();
            if tree.0 > height {
                height = tree.0;
                tree.1 = true;
            }
        }
        height = -1;
        for y in (0..=y_max).rev() {
            let mut tree = map.get_mut(&(x, y)).unwrap();
            if tree.0 > height {
                height = tree.0;
                tree.1 = true;
            }
        }
    }

    for y in 0..=y_max {
        let mut height = -1;
        for x in 0..=x_max {
            let mut tree = map.get_mut(&(x, y)).unwrap();
            if tree.0 > height {
                height = tree.0;
                tree.1 = true;
            }
        }
        height = -1;
        for x in (0..=x_max).rev() {
            let mut tree = map.get_mut(&(x, y)).unwrap();
            if tree.0 > height {
                height = tree.0;
                tree.1 = true;
            }
        }
    }

    let res = map
        .iter()
        .filter(|(_, (_, b))| *b)
        .fold(0, |acc, _| acc + 1);

    println!("res: {res}");

    Ok(())
}
