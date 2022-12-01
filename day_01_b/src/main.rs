use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut elf_calories = Vec::new();

    for elf in input.split("\n\n") {
        let mut calorie_count = 0;

        for line in elf.lines() {
            calorie_count += line.parse::<i32>()?;
        }

        elf_calories.push(calorie_count);
    }

    elf_calories.sort();
    elf_calories.reverse();

    println!("{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);

    Ok(())
}
