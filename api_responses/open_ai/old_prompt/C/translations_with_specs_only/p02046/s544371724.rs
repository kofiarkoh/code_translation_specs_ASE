use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Unwrap the line to get the string
        let line = line.unwrap();
        
        // Split the line into parts and collect them into a vector
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Check if we have exactly two parts
        if parts.len() != 2 {
            // Undefined behavior for invalid input; we do not handle it gracefully
            continue; // Simply skip invalid input
        }

        // Parse the two integers from the input
        let x: i32 = parts[0].parse().unwrap(); // Unwrap assumes valid input as per spec
        let y: i32 = parts[1].parse().unwrap(); // Unwrap assumes valid input as per spec

        // Compute the absolute values
        let abs_x = x.abs();
        let abs_y = y.abs();

        // Compute the value t based on the absolute values and their parity
        let t = if abs_x % 2 == 0 && abs_y % 2 == 0 {
            abs_x + abs_y
        } else if abs_x % 2 != 0 && abs_y % 2 != 0 {
            abs_x - abs_y
        } else {
            abs_x * abs_y
        };

        // Print the computed value t
        println!("{}", t);
    }
}
