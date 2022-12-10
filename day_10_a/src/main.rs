use std::{error::Error, fs};

fn maybe_add_signal_strength(cycle: i32, x: i32) -> i32 {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        return cycle * x;
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut cycle = 0;
    let mut x = 1;

    let mut signal_strength = 0;

    for line in input.lines() {
        cycle += 1;
        signal_strength += maybe_add_signal_strength(cycle, x);
        if line != "noop" {
            cycle += 1;
            signal_strength += maybe_add_signal_strength(cycle, x);

            let parts = line.split_once(' ').unwrap();
            x += parts.1.parse::<i32>().unwrap();
        }
    }

    println!("{signal_strength}");

    Ok(())
}
