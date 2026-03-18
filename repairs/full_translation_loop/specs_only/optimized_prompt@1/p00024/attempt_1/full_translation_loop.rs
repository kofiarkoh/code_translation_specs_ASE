use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Read the line and handle potential errors
        if let Ok(value) = line {
            // Attempt to parse the line as a floating-point number
            if let Ok(velocity) = value.parse::<f64>() {
                // Compute the integer output based on the input velocity
                // The formula is not specified, so we will use a placeholder
                // Here we assume a simple transformation for demonstration
                let computed_value = (velocity * 10.0).round() as i32;

                // Ensure the output is always >= 1
                let output_value = if computed_value < 1 { 1 } else { computed_value };

                // Print the computed integer value
                println!("{}", output_value);
            }
            // If parsing fails, we simply continue to the next input
            // as per the specification's error handling
        }
    }
}
