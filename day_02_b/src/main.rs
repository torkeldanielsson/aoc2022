use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut score = 0;

    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let (them, us) = (parts[0], parts[1]);
        score += match us {
            "X" => match them {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            },
            "Y" => {
                3 + match them {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => 0,
                }
            }
            "Z" => {
                6 + match them {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => 0,
                }
            }
            _ => 0,
        };
    }

    println!("{score}");

    Ok(())
}
