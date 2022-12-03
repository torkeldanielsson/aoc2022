use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut score = 0;

    for line in input.lines() {
        let (a, b) = line.split_at(line.len() / 2);

        'outer: for al in a.chars() {
            for bl in b.chars() {
                if al == bl {
                    let ascii = al as u32;
                    if ascii < 97 {
                        score += ascii - 38;
                    } else {
                        score += ascii - 96;
                    }
                    break 'outer;
                }
            }
        }
    }

    println!("{score}");

    Ok(())
}
