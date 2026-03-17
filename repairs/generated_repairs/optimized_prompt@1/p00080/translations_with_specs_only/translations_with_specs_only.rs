use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Handle potential I/O errors
        match line {
            Ok(input) => {
                // Parse the input as an integer
                if let Ok(value) = input.trim().parse::<i32>() {
                    // Check for termination signal
                    if value == -1 {
                        break; // Exit the loop on termination signal
                    }
                    // Compute the cube root
                    let cube_root = cube_root(value);
                    // Print the result formatted to six decimal places
                    println!("{:.6}", cube_root);
                }
                // If parsing fails, we do nothing as per the specification
            }
            Err(_) => {
                // Handle potential I/O errors gracefully
                // Specification states behavior is not defined for invalid input
                continue;
            }
        }
    }
}

// Function to compute the cube root of an integer
fn cube_root(n: i32) -> f64 {
    // Use f64 for cube root calculation to ensure precision
    (n as f64).cbrt() // cbrt() computes the cube root
}
