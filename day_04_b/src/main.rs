use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut score = 0;

    for line in input.lines() {
        let parts = line.split(',').collect::<Vec<_>>();
        let e1 = parts[0].split('-').collect::<Vec<_>>();
        let e2 = parts[1].split('-').collect::<Vec<_>>();
        let a = e1[0].parse::<i32>()?;
        let b = e1[1].parse::<i32>()?;
        let c = e2[0].parse::<i32>()?;
        let d = e2[1].parse::<i32>()?;

        if (a <= c && c <= b) || (a <= d && d <= b) || (c <= a && a <= d) || (c <= b && b <= d) {
            score += 1;
        }
    }

    println!("{score}");

    Ok(())
}
