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
        let count = parts[1].parse::<usize>()?;
        let src = parts[3].parse::<usize>()? - 1;
        let dst = parts[5].parse::<usize>()? - 1;

        let new_len = stacks[src as usize].len() - count;
        let tmp = stacks[src as usize].split_off(new_len);
        stacks[dst as usize].extend(tmp);
    }

    for i in 0..stacks.len() {
        print!("{}", stacks[i].last().unwrap());
    }

    println!();

    Ok(())
}
