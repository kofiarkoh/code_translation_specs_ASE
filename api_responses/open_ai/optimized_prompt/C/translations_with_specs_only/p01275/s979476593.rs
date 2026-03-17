fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();

        // Termination condition: if n is 0, we stop processing.
        if n == 0 {
            break;
        }

        let mut init = String::new();
        let mut unlock = String::new();

        // Read the initial and unlock sequences
        io::stdin().read_line(&mut init).unwrap();
        io::stdin().read_line(&mut unlock).unwrap();

        // Trim newline characters
        let init = init.trim();
        let unlock = unlock.trim();

        // Ensure the input strings are of the expected length and contain only digits
        if init.len() != n || unlock.len() != n || !init.chars().all(|c| c.is_digit(10)) || !unlock.chars().all(|c| c.is_digit(10)) {
            // Behavior is not defined for invalid input, so we just return.
            return;
        }

        let rotations = calculate_min_rotations(init, unlock);
        println!("{}", rotations);
    }
}

// Function to calculate the minimum rotations needed to match init to unlock
fn calculate_min_rotations(init: &str, unlock: &str) -> u32 {
    let n = init.len();
    let mut total_rotations = 0;

    for i in 0..n {
        let init_digit = init.chars().nth(i).unwrap().to_digit(10).unwrap();
        let unlock_digit = unlock.chars().nth(i).unwrap().to_digit(10).unwrap();

        // Calculate the minimum rotations needed for each digit
        let forward_rotations = (unlock_digit as i32 - init_digit as i32 + 10) % 10;
        let backward_rotations = (init_digit as i32 - unlock_digit as i32 + 10) % 10;

        // Add the minimum of the two to the total rotations
        total_rotations += forward_rotations.min(backward_rotations) as u32;
    }

    total_rotations
}
