use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut stacks = {
        let stacks = parts[0].lines().collect::<Vec<_>>();
        let mut res = Vec::new();
        let stack_count = (stacks[0].len() + 1) / 4;
        for _ in 0..stack_count {
            res.push(Vec::new());
        }

        for i in 0..stacks.len() - 1 {
            let line = stacks[i].chars().collect::<Vec<char>>();
            for pos in 0..stack_count {
                let c = line[pos * 4 + 1];
                if c != ' ' {
                    res[pos].push(c);
                }
            }
        }

        res
    };

    for s in &mut stacks {
        s.reverse();
    }

    for line in parts[1].lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<i32>()?;
        let src = parts[3].parse::<i32>()? - 1;
        let dst = parts[5].parse::<i32>()? - 1;

        for _ in 0..count {
            let tmp = stacks[src as usize].pop();
            if let Some(c) = tmp {
                stacks[dst as usize].push(c);
            } else {
                println!("error?");
            }
        }
    }

    for i in 0..stacks.len() {
        print!("{}", stacks[i].last().unwrap());
    }

    println!();

    Ok(())
}
