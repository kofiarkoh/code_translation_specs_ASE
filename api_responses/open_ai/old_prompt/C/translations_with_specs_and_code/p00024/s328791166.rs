use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a float
                if let Ok(v) = input.trim().parse::<f32>() {
                    // Ensure v is non-negative as per the specification
                    if v >= 0.0 {
                        // Calculate the derived integer value
                        let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
                        let result = n as i32 + 1; // Convert to integer and add 1
                        println!("{}", result); // Output the result
                    }
                }
                // If parsing fails or v is negative, the behavior is undefined as per the spec
            }
            Err(_) => {
                // Handle potential I/O errors, but the spec states undefined behavior for invalid input
                // Thus, we do not need to handle this case explicitly.
            }
        }
    }
}
