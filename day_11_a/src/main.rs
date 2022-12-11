use std::{error::Error, fs};

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test: i64,
    throw_targets: (usize, usize),
    inspect_count: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut monkeys = Vec::new();

    for monkey in input.split("\n\n") {
        let mut lines = monkey.lines();
        lines.next();
        let items = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&s| s.to_owned().replace(',', ""))
            .collect::<Vec<String>>()[2..]
            .to_owned()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let operation = lines.next().unwrap().replace("  Operation: new = ", "");
        let test = lines
            .next()
            .unwrap()
            .replace("  Test: divisible by ", "")
            .parse::<i64>()
            .unwrap();
        let throw_true = lines
            .next()
            .unwrap()
            .replace("    If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let throw_false = lines
            .next()
            .unwrap()
            .replace("    If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            throw_targets: (throw_true, throw_false),
            inspect_count: 0,
        });
    }

    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            monkeys[i].items = monkeys[i].items.clone().into_iter().rev().collect();
            while let Some(item) = monkeys[i].items.pop().to_owned().as_mut() {
                monkeys[i].inspect_count += 1;
                if monkeys[i].operation == "old * old" {
                    *item = *item * *item;
                } else if monkeys[i].operation.starts_with("old +") {
                    let plus = monkeys[i]
                        .operation
                        .clone()
                        .replace("old + ", "")
                        .parse::<i64>()
                        .unwrap();
                    *item += plus;
                } else if monkeys[i].operation.starts_with("old *") {
                    let mul = monkeys[i]
                        .operation
                        .clone()
                        .replace("old * ", "")
                        .parse::<i64>()
                        .unwrap();
                    *item *= mul;
                } else {
                    panic!("?");
                }
                *item /= 3;
                if (*item % monkeys[i].test) == 0 {
                    let target = monkeys[i].throw_targets.0;
                    monkeys[target].items.push(*item);
                } else {
                    let target = monkeys[i].throw_targets.1;
                    monkeys[target].items.push(*item);
                }
            }
        }
    }

    let mut res: Vec<i32> = monkeys
        .iter()
        .map(|m| m.inspect_count)
        .collect::<Vec<i32>>();
    res.sort();
    let res: Vec<i32> = res.into_iter().rev().collect();

    println!("{}", res[0] * res[1]);

    Ok(())
}
