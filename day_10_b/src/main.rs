use std::{error::Error, fs};

fn process(cycle: i32, x: i32) {
    if ((cycle % 40) - x).abs() <= 1 {
        print!("#");
    } else {
        print!(".")
    }
    if (cycle + 1) % 40 == 0 && cycle != 0 {
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut cycle: i32 = 0;
    let mut x = 1;

    for line in input.lines() {
        process(cycle, x);
        cycle += 1;

        if line != "noop" {
            process(cycle, x);
            cycle += 1;

            let parts = line.split_once(' ').unwrap();
            x += parts.1.parse::<i32>().unwrap();
        }
    }

    Ok(())
}
