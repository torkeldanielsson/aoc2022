use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut score = 0;

    for line in input.lines().collect::<Vec<&str>>().chunks(3) {
        let a = line[0];
        let b = line[1];
        let c = line[2];

        'outer: for al in a.chars() {
            for bl in b.chars() {
                for cl in c.chars() {
                    if al == bl && bl == cl {
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
    }

    println!("{score}");

    Ok(())
}
