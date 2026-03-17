use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line of input
        let input = line.expect("Failed to read line");
        // Parse the input as an integer
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip to the next input
        };

        // Check for termination signal
        if number == -1 {
            break; // Exit the loop on termination signal
        }

        // Only process non-negative integers for cube root calculation
        if number >= 0 {
            let cube_root = (number as f64).cbrt(); // Compute the cube root
            // Print the result formatted to six decimal places
            println!("{:.6}", cube_root);
        }
    }
}
