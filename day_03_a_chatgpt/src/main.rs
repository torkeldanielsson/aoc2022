use std::io;

// Helper function to calculate the priority of an item type based on its letter
fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn main() {
    // Read in the rucksack contents from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Split the input string into a vector of rucksacks
    let rucksacks: Vec<&str> = input.split(' ').collect();

    // Initialize a variable to keep track of the sum of priorities
    let mut sum = 0;

    // Iterate over the rucksacks
    for rucksack in rucksacks {
        // Split each rucksack into its two compartments
        let compartments: Vec<&str> = rucksack.split('/');

        // Initialize a variable to keep track of the item types that appear in both compartments
        let mut common_types = Vec::new();

        // Iterate over the first compartment
        for item in compartments[0].chars() {
            // If the item type also appears in the second compartment, add it to the common types vector
            if compartments[1].contains(item) {
                common_types.push(item);
            }
        }

        // If there is only one item type that appears in both compartments, add its priority to the sum
        if common_types.len() == 1 {
            sum += priority(common_types[0]);
        }
    }

    // Print out the sum of priorities
    println!("{}", sum);
}