use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    // Check if we have exactly three integers
    let values: Vec<i32> = match input {
        Some(Ok(line)) => line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect(),
        _ => return, // No output if input is not valid
    };

    if values.len() != 3 {
        return; // No output if fewer than three integers are provided
    }

    // Ensure input values are within the range 1 to 10
    if values.iter().any(|&v| v < 1 || v > 10) {
        return; // No output for out-of-range values
    }

    // Calculate the sum of the input values
    let sum: i32 = values.iter().sum();
    let mut valid_count = 0;

    // Check for valid integers from 1 to 10, excluding input values
    for i in 1..=10 {
        if !values.contains(&i) && sum + i <= 20 {
            valid_count += 1;
        }
    }

    // Output "YES" if there are at least four valid options, otherwise "NO"
    if valid_count >= 4 {
        println!("YES");
    } else {
        println!("NO");
    }
}
