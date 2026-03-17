fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        // Read the length of the digit strings
        let length: usize = parts.next().unwrap().parse().unwrap();
        
        // Terminate if the length is zero
        if length == 0 {
            break;
        }

        // Read the initial and target states
        let initial = parts.next().unwrap();
        let target = parts.next().unwrap();

        // Check if the input strings are of the specified length
        if initial.len() != length || target.len() != length {
            // Behavior is not defined; we will just skip this case
            continue;
        }

        // Calculate the minimum number of rotations
        let rotations = calculate_min_rotations(initial, target);
        println!("{}", rotations);
    }
}

// Function to calculate the minimum number of rotations
fn calculate_min_rotations(initial: &str, target: &str) -> usize {
    // If the states are identical, return zero
    if initial == target {
        return 0;
    }

    let length = initial.len();
    let mut min_rotations = usize::MAX;

    // Check all possible rotations
    for i in 0..length {
        let rotated = initial[i..].to_string() + &initial[..i]; // Rotate the string
        let mut current_rotations = 0;

        // Calculate the number of rotations needed for each digit
        for (a, b) in rotated.chars().zip(target.chars()) {
            let diff = (b.to_digit(10).unwrap() as isize - a.to_digit(10).unwrap() as isize).rem_euclid(10);
            current_rotations += diff as usize; // Accumulate the rotations
        }

        // Update the minimum rotations found
        min_rotations = min_rotations.min(current_rotations);
    }

    min_rotations
}
