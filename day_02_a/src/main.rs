use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut score = 0;

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let (them, us) = (parts[0], parts[1]);
        score += match us {
            "X" => {
                1 + match them {
                    "A" => 3,
                    "C" => 6,
                    _ => 0,
                }
            }
            "Y" => {
                2 + match them {
                    "B" => 3,
                    "A" => 6,
                    _ => 0,
                }
            }
            "Z" => {
                3 + match them {
                    "C" => 3,
                    "B" => 6,
                    _ => 0,
                }
            }
            _ => 0,
        };
    }

    println!("{score}");

    Ok(())
}
