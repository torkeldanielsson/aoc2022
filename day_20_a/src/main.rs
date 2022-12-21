use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    let mut numbers = fs::read_to_string("test")?
        .lines()
        .map(|s| {
            i += 1;
            (s.parse::<i32>().unwrap(), i)
        })
        .collect::<Vec<_>>();

    for i in 1..=i {
        let mut index = 0;
        for ii in 0..numbers.len() {
            if numbers[ii].1 == i {
                index = ii;
                break;
            }
        }

        let n = numbers.remove(index);
        let mut pos = n.0 + index as i32;
        while pos < 0 {
            pos += numbers.len() as i32;
        }
        pos %= numbers.len() as i32;
        numbers.insert(pos as usize, (n.0, i));
        //println!("{i} {index} {} {i}->{pos} => {numbers:?}", n.0);
    }

    for i in 1..=i {
        if numbers[i].0 == 0 {
            let num_len = numbers.len();
            let v1 = numbers[(i + 1000) % num_len].0;
            let v2 = numbers[(i + 2000) % num_len].0;
            let v3 = numbers[(i + 3000) % num_len].0;

            println!("{v1}, {v2}, {v3}: {}", v1 + v2 + v3);

            break;
        }
    }

    Ok(())
}
