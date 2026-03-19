use std::io::{self, BufRead};

fn main() {
    let mut railroad: Vec<i32> = Vec::new(); // Vector to store car identifiers
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let car: i32 = line.unwrap().trim().parse().unwrap(); // Read and parse input

        if car == 0 { // Car exiting
            if let Some(removed_car) = railroad.pop() { // Remove the last car if available
                println!("{}", removed_car); // Output the identifier of the car that exited
            }
        } else { // Car entering
            if railroad.len() < 10 { // Check if we can add more cars
                railroad.push(car); // Add the car to storage
            }
            // If we reach the limit of 10 cars, we do nothing (undefined behavior as per spec)
        }
    }
}
