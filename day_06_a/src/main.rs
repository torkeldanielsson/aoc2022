use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?.chars().collect::<Vec<char>>();

    for i in 3..input.len() {
        if input[i - 3] != input[i - 2]
            && input[i - 3] != input[i - 1]
            && input[i - 3] != input[i]
            && input[i - 2] != input[i - 1]
            && input[i - 2] != input[i]
            && input[i - 1] != input[i]
        {
            println!("{}", i + 1);
            return Ok(());
        }
    }

    Ok(())
}
