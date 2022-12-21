use std::{error::Error, fs};

// root: tcmj + qggp
// humn: 3640

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

    let mut new_known_numbers = known_numbers.clone();
    let mut new_equations = equations.clone();
    new_known_numbers.insert("humn", -700);

    while !new_equations.is_empty() {
        for i in 0..new_equations.len() {
            let eq = &new_equations[i];
            if let (Some(a), Some(b)) = (new_known_numbers.get(eq.1), new_known_numbers.get(eq.3)) {
                new_known_numbers.insert(
                    eq.0,
                    match eq.2 {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => panic!(),
                    },
                );
                new_equations.remove(i);
                break;
            }
        }
    }

    println!(
        "humn: {} => {} == {}",
        new_known_numbers["humn"], new_known_numbers["tcmj"], new_known_numbers["qggp"]
    );

    Ok(())
}
// 792784087587
// 9244733517335
