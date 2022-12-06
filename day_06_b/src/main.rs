use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?.chars().collect::<Vec<char>>();

    'outer: for i in 13..input.len() {
        for a in 0..13 {
            for b in (a + 1)..14 {
                if input[i - a] == input[i - b] {
                    continue 'outer;
                }
            }
        }

        println!("{}", i + 1);
        return Ok(());
    }

    Ok(())
}
