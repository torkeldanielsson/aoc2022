use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = fs::read_to_string("input")?;

    let mut sum = 0;

    for line in input.lines() {
        let mut r = 0;
        for (i, c) in line.chars().rev().enumerate() {
            let v = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            r += 5_i64.pow(i as u32) * v;
        }

        sum += r;
    }

    let mut result = String::new();
    let mut num = sum;

    while num > 0 {
        let rem = ((num + 2) % 5) - 2;
        result.push(match rem {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        });
        num = (num + 2) / 5;
    }

    let res = result.chars().rev().collect::<String>();

    println!("res: {res} {sum}, {} us", t.elapsed().as_micros());

    Ok(())
}
