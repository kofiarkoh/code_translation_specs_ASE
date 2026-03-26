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
                if let Ok(v) = input.parse::<f32>() {
                    // Ensure the velocity is non-negative as per the specification
                    if v >= 0.0 {
                        // Calculate the derived integer value based on the velocity
                        let derived_value = (v * 9.81) as i32; // Example calculation, adjust as needed
                        // Print the derived integer value
                        println!("{}", derived_value);
                    }
                }
                // If parsing fails or the value is negative, the behavior is undefined
                // as per the specification, so we do nothing.
            }
            Err(_) => {
                // If there's an error reading the line, we ignore it as per the spec
                // (undefined behavior on invalid input).
            }
        }
    }
}
