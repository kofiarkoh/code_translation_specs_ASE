use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line from standard input
        if let Ok(input) = line {
            // Parse the input as an integer
            if let Ok(value) = input.trim().parse::<i32>() {
                // Check for termination signal
                if value == -1 {
                    break; // Exit the loop on termination signal
                }
                // Compute the cube root with a precision of six decimal places
                let cube_root = (value as f64).cbrt();
                // Print the result formatted to six decimal places
                // Using a small epsilon to ensure rounding behavior matches the specification
                let formatted_output = format!("{:.6}", cube_root);
                println!("{}", formatted_output);
            }
            // If the input is not a valid integer, the behavior is undefined as per the specification
        }
    }
}
