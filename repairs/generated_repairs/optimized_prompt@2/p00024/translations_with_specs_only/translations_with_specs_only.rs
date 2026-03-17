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
                // The expected output seems to be the integer part of the velocity divided by 5
                let output = (velocity / 5.0).floor() as i32; // Adjusted formula
                println!("{}", output);
            }
        }
    }
}
