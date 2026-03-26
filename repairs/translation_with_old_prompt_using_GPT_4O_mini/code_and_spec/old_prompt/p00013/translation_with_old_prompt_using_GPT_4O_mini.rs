use std::io::{self, BufRead};

fn main() {
    let mut railroad = [0; 10]; // Array to hold car identifiers
    let mut i = 0; // Index to track the number of cars

    // Read from standard input until EOF
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Handle potential I/O errors
        let line = line.expect("Failed to read line");
        // Attempt to parse the input as an integer
        if let Ok(car) = line.trim().parse::<i32>() {
            if car == 0 { // Car exiting
                if i > 0 { // Ensure we don't go out of bounds
                    i -= 1;
                    println!("{}", railroad[i]); // Output the identifier of the exiting car
                }
            } else { // Car entering
                if i < 10 { // Ensure we don't exceed the array bounds
                    railroad[i] = car; // Add the car identifier
                    i += 1; // Increment the index
                }
            }
        }
    }

    // Return 0 upon successful completion
    std::process::exit(0);
}
