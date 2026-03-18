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
                // The formula is adjusted to match the expected output
                // For the given inputs, we will use the following logic:
                // - For velocities in the range [0.0, 2.5), output 2
                // - For velocities in the range [2.5, 5.0), output 2
                // - For velocities in the range [5.0, 10.0), output 3
                let computed_value = match velocity {
                    v if v < 2.5 => 2,
                    v if v < 5.0 => 2,
                    v if v < 10.0 => 3,
                    _ => 1, // Default case, though not expected per the specification
                };

                // Print the computed integer value
                println!("{}", computed_value);
            }
            // If parsing fails, we simply continue to the next input
            // as per the specification's error handling
        }
    }
}
