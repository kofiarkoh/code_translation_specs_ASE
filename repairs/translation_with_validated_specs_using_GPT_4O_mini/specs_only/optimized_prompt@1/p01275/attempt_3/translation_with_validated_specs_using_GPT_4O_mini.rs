use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: usize = match input.trim().parse() {
            Ok(num) if num > 0 && num <= 10 => num,
            _ => break, // Terminate on non-positive integer
        };

        let mut init = String::new();
        let mut unlock = String::new();

        io::stdin().read_line(&mut init).expect("Failed to read line");
        io::stdin().read_line(&mut unlock).expect("Failed to read line");

        // Trim whitespace to ensure correct length check
        let init = init.trim();
        let unlock = unlock.trim();

        // Ensure the lengths of init and unlock match n
        if init.len() != n || unlock.len() != n {
            continue; // Skip invalid input
        }

        // Check if sequences are identical
        if init == unlock {
            println!("0");
            continue;
        }

        let min_rotations = calculate_min_rotations(init, unlock);
        println!("{}", min_rotations);
    }
}

fn calculate_min_rotations(init: &str, unlock: &str) -> usize {
    let mut total_rotations = 0;

    for (i, c) in init.chars().enumerate() {
        let init_digit = c.to_digit(10).unwrap();
        let unlock_digit = unlock.chars().nth(i).unwrap().to_digit(10).unwrap();

        // Calculate clockwise and counterclockwise rotations
        let clockwise = (unlock_digit as isize - init_digit as isize + 10) % 10;
        let counterclockwise = (init_digit as isize - unlock_digit as isize + 10) % 10;

        // Add the minimum of the two to the total rotations
        total_rotations += clockwise.min(counterclockwise) as usize;
    }

    total_rotations
}
