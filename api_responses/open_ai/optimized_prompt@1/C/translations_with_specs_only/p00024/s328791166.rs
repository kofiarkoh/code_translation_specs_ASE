use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Read the line and handle potential errors
        if let Ok(input) = line {
            // Attempt to parse the input as a floating-point number
            if let Ok(velocity) = input.parse::<f64>() {
                // Compute the integer output based on the velocity
                // The formula is not specified, so we will use a placeholder
                // Here we assume a simple transformation for demonstration
                // The output must be >= 1, so we use max to ensure this
                let output = (velocity * 10.0).round() as i32; // Placeholder formula
                let output = output.max(1); // Ensure output is at least 1
                println!("{}", output);
            }
            // If the input is not a valid floating-point number, we ignore it
            // as per the specification, which states behavior is undefined
            // for invalid inputs.
        }
    }
}
