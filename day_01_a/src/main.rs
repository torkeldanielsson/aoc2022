use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut highest_calorie_count = 0;

    for elf in input.split("\n\n") {
        let mut calorie_count = 0;

        for line in elf.lines() {
            calorie_count += line.parse::<i32>()?;
        }

        if calorie_count > highest_calorie_count {
            highest_calorie_count = calorie_count;
        }
    }

    println!("{}", highest_calorie_count);

    Ok(())
}
