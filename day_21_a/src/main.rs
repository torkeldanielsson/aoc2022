use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?.replace(':', "");

    let mut known_numbers = std::collections::BTreeMap::new();
    let mut equations = Vec::new();

    for line in input
        .lines()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
    {
        match line.len() {
            2 => {
                known_numbers.insert(line[0], line[1].parse::<i64>().unwrap());
            }
            4 => {
                equations.push((line[0], line[1], line[2], line[3]));
            }
            _ => panic!(),
        }
    }

    while !equations.is_empty() {
        for i in 0..equations.len() {
            let eq = &equations[i];
            if let (Some(a), Some(b)) = (known_numbers.get(eq.1), known_numbers.get(eq.3)) {
                known_numbers.insert(
                    eq.0,
                    match eq.2 {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => panic!(),
                    },
                );
                equations.remove(i);
                break;
            }
        }
    }

    println!("{}", known_numbers["root"]);

    Ok(())
}
