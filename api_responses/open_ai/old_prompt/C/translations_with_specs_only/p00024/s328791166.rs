use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Continuously read lines from standard input until EOF
    for line in reader.lines() {
        // Handle potential errors in reading lines
        match line {
            Ok(input) => {
                // Attempt to parse the input as a floating-point number
                if let Ok(v) = input.trim().parse::<f32>() {
                    // Ensure the velocity is non-negative as per the specification
                    if v >= 0.0 {
                        // Calculate the derived integer value based on the velocity
                        let derived_value = (v * 9.81).round() as i32; // Example calculation
                        // Output the derived integer value
                        println!("{}", derived_value);
                    }
                    // If v is negative, we do nothing (undefined behavior)
                }
                // If parsing fails, we do nothing (undefined behavior)
            }
            Err(_) => {
                // If there's an error reading the line, we do nothing (undefined behavior)
            }
        }
    }
}
