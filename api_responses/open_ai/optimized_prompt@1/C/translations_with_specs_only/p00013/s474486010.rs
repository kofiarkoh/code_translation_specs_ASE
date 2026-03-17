use std::io::{self, BufRead};

fn main() {
    // Create a vector to store car identifiers, with a maximum capacity of 10
    let mut storage: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read input line by line
    for line in handle.lines() {
        // Unwrap the line to get the string and parse it as an integer
        if let Ok(input) = line {
            if let Ok(car_id) = input.trim().parse::<i32>() {
                // If the input is zero, a car is exiting
                if car_id == 0 {
                    // Only remove a car if there is at least one in storage
                    if let Some(exited_car) = storage.pop() {
                        // Output the identifier of the car that exited
                        println!("{}", exited_car);
                    }
                } else {
                    // If the input is a non-zero integer, add the car to storage
                    // Check if we can add more cars (max 10)
                    if storage.len() < 10 {
                        storage.push(car_id);
                    }
                    // If we exceed the limit, the behavior is undefined as per spec
                }
            }
            // Invalid input handling is not defined in the specification
        }
    }
}
