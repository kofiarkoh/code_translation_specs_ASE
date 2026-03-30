use std::io::{self, BufRead}; // Import necessary traits for reading input

struct RailroadCarManager {
    storage: Vec<i32>, // Storage for cars, using i32 to represent car IDs
    capacity: usize,    // Maximum number of cars that can be stored
}

impl RailroadCarManager {
    fn new(capacity: usize) -> Self {
        RailroadCarManager {
            storage: Vec::with_capacity(capacity), // Initialize storage with the given capacity
            capacity,
        }
    }

    fn process_input(&mut self, input: i32) {
        if input > 0 {
            // If the input is a positive integer, add it to storage if there's capacity
            if self.storage.len() < self.capacity {
                self.storage.push(input);
            }
            // If storage is full, we ignore the input as per the specification
        } else if input == 0 {
            // If the input is zero, we attempt to remove the last car from storage
            if let Some(last_car) = self.storage.pop() {
                // Output the last car that exited
                println!("{}", last_car);
            }
            // If storage is empty, we do nothing (no output)
        }
    }
}

fn main() {
    let mut manager = RailroadCarManager::new(10); // Example capacity of 10 cars

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(car_id) = input.trim().parse::<i32>() {
                manager.process_input(car_id);
            }
        }
    }
}
