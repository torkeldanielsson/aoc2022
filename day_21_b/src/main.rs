use std::{collections::BTreeMap, error::Error, fs};

// root: tcmj + qggp

fn pull(
    tag: &str,
    equations: &BTreeMap<&str, (&str, &str, &str)>,
    known_numbers: &BTreeMap<&str, i64>,
) -> String {
    if let Some(n) = known_numbers.get(tag) {
        return format!("{n}");
    }
    if let Some(eq) = equations.get(tag) {
        return format!(
            "({} {} {})",
            pull(eq.0, equations, known_numbers),
            eq.1,
            pull(eq.2, equations, known_numbers),
        );
    }
    panic!()
}

fn main() -> Result<(), Box<dyn Error>> {
    /*
    let input = fs::read_to_string("input")?.replace(':', "");

    let mut known_numbers = BTreeMap::new();
    let mut v_equations = Vec::new();

    for line in input
        .lines()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
    {
        match line.len() {
            2 => {
                known_numbers.insert(line[0], line[1].parse::<i64>().unwrap());
            }
            4 => {
                v_equations.push((line[0], line[1], line[2], line[3]));
            }
            _ => panic!(),
        }
    }

    loop {
        let mut simplified = false;

        for i in 0..v_equations.len() {
            let eq = &v_equations[i];
            if eq.1 == "humn" || eq.3 == "humn" {
                continue;
            }
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
                simplified = true;
                v_equations.remove(i);
                break;
            }
        }

        if !simplified {
            break;
        }
    }

    let mut equations = BTreeMap::new();
    for ve in v_equations {
        equations.insert(ve.0, (ve.1, ve.2, ve.3));
    }

    println!("tcmj: {}", pull("tcmj", &equations, &known_numbers));
    println!("qggp: {}", pull("qggp", &equations, &known_numbers));
    */

    for i in 3272260910000..3272260920000 {
        let num: i64 = (634
            + (101692068627800
                - (((((2
                    * (516
                        + ((868
                            + (((((2
                                * (((((99
                                    + ((((2
                                        * ((((((((((((((5
                                            * (((268
                                                + (((76
                                                    + ((((((((837
                                                        + ((((337
                                                            + ((((2
                                                                * (((674
                                                                    + ((((137
                                                                        + i)
                                                                        / 3)
                                                                        - 467)
                                                                        * 81))
                                                                    / 2)
                                                                    - 387))
                                                                - 865)
                                                                + 365)
                                                                / 8))
                                                            * 11)
                                                            - 215)
                                                            / 4))
                                                        * 4)
                                                        + 998)
                                                        * 2)
                                                        - 794)
                                                        / 2)
                                                        - 602)
                                                        * 2))
                                                    / 2)
                                                    - 414))
                                                / 7)
                                                + 614))
                                            + 81)
                                            * 2)
                                            - 562)
                                            / 6)
                                            - 829)
                                            * 3)
                                            + 173)
                                            / 7)
                                            + 871)
                                            / 6)
                                            - 888)
                                            * 40)
                                            - 128))
                                        + 519)
                                        + 941)
                                        / 12))
                                    * 9)
                                    - 582)
                                    / 4)
                                    + 844))
                                - 191)
                                / 3)
                                - 386)
                                / 5))
                            * 15)))
                    - 338)
                    / 4)
                    + 88)
                    * 3)))
            / 11;

            println!("{i}: {num}: {}", num - 792784087587);
    }

    Ok(())
}
