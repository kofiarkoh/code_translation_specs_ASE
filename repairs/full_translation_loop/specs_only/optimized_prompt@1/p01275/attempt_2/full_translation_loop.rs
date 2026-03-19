use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        // Read the length of the digit strings
        let length: usize = parts.next().unwrap().parse().unwrap();
        
        // Termination condition: if length is zero, we stop processing
        if length == 0 {
            break;
        }

        // Read the initial and target states
        let initial = parts.next().unwrap();
        let target = parts.next().unwrap();

        // Ensure the strings are of the specified length
        if initial.len() != length || target.len() != length {
            // Behavior is not defined; we will just continue to the next iteration
            continue;
        }

        // Calculate the minimum rotations required
        let rotations = calculate_min_rotations(initial, target);
        println!("{}", rotations);
    }
}

// Function to calculate the minimum rotations needed to match the initial state to the target state
fn calculate_min_rotations(initial: &str, target: &str) -> usize {
    // If the states are identical, return zero
    if initial == target {
        return 0;
    }

    let mut min_rotations = usize::MAX;

    // Check all possible rotations
    for i in 0..initial.len() {
        // Rotate the string by slicing and concatenating
        let rotated = format!("{}{}", &initial[i..], &initial[..i]);
        let rotations = calculate_rotation_count(&rotated, target);
        min_rotations = min_rotations.min(rotations);
    }

    min_rotations
}

// Function to calculate the rotation count between two strings
fn calculate_rotation_count(rotated: &str, target: &str) -> usize {
    let mut count = 0;

    for (r, t) in rotated.chars().zip(target.chars()) {
        // Calculate the difference and ensure it's non-negative and <= 9
        let diff = (r.to_digit(10).unwrap() as isize - t.to_digit(10).unwrap() as isize).rem_euclid(10);
        count += diff as usize;
    }

    count
}
