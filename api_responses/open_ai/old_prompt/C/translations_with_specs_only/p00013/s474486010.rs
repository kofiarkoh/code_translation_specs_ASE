use std::io::{self, BufRead};

fn main() -> i32 {
    // Create a fixed-size array to hold up to 10 car identifiers
    let mut railroad: [i32; 10] = [0; 10];
    let mut i = 0; // Index to track the number of cars in the railroad

    // Read input until EOF
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Unwrap the line to get the input string
        let line = line.unwrap();
        // Parse the input as an integer
        if let Ok(car_id) = line.trim().parse::<i32>() {
            if car_id > 0 {
                // A positive integer indicates a car entering
                if i < 10 {
                    // Only add if there's space in the array
                    railroad[i] = car_id;
                    i += 1; // Increment the count of cars
                } else {
                    // If the array is full, overwrite the oldest entry
                    railroad[i % 10] = car_id; // This simulates overwriting
                    i += 1; // Increment the count of cars
                }
            } else if car_id == 0 {
                // A zero indicates a car exiting
                if i > 0 {
                    // Only exit if there are cars in the railroad
                    i -= 1; // Decrement the count of cars
                    println!("{}", railroad[i]); // Output the identifier of the car that exits
                }
            }
            // If car_id < 0, we do nothing (invalid input)
        }
    }

    // Return 0 upon successful completion
    0
}
