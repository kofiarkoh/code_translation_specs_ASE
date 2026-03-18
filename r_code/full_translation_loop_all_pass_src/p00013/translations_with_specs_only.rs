use std::io::{self, BufRead};

fn main() {
    // Create a vector to store the identifiers of the cars in the storage.
    let mut storage: Vec<i32> = Vec::new();

    // Read from standard input line by line.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Unwrap the line to get the string and parse it as an integer.
        if let Ok(input) = line {
            if let Ok(car_id) = input.trim().parse::<i32>() {
                // If the input is zero, a car is exiting the storage.
                if car_id == 0 {
                    // Only pop from storage if there is at least one car.
                    if let Some(exited_car) = storage.pop() {
                        // Output the identifier of the car that exited.
                        println!("{}", exited_car);
                    }
                } else {
                    // If the input is a non-zero integer, add the car to storage
                    // only if we have not exceeded the maximum capacity of 10.
                    if storage.len() < 10 {
                        storage.push(car_id);
                    }
                    // If we exceed the capacity, the behavior is undefined as per the spec.
                }
            }
        }
    }
}
