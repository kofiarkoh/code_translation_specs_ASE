use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Handle potential I/O errors
        if let Ok(input) = line {
            // Attempt to parse the input as a floating-point number
            if let Ok(velocity) = input.parse::<f64>() {
                // Compute the integer output based on the velocity
                // The formula is not specified, so we will use a placeholder
                // Here we assume a simple transformation for demonstration
                let output = (velocity * 10.0).round() as i32; // Example transformation

                // Ensure the output is at least 1
                let output = if output < 1 { 1 } else { output };

                // Print the computed integer output
                println!("{}", output);
            }
            // If parsing fails, we ignore the input as per the specification
        }
    }
}
