use std::io::{self, BufRead};

fn main() {
    // Read from standard input line by line
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Unwrap the line to get the string, assuming valid input as per specification
        let line = line.unwrap();
        
        // Parse the line as a floating-point number
        if let Ok(velocity) = line.parse::<f64>() {
            // Compute the derived integer value based on the given formula
            let n = ((4.9 * velocity.powi(2)) / (9.8 * 9.8) + 5.0).floor() / 5.0 + 1.0;
            // Output the integer value, ensuring it is at least 1
            println!("{}", n as i32);
        }
        // Undefined behavior for invalid input is not handled as per specification
    }
}
