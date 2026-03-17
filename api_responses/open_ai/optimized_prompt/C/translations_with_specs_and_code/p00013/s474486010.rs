use std::io::{self, BufRead};

fn main() {
    let mut railroad: [i32; 10] = [0; 10]; // Fixed-size array to hold car numbers
    let mut i = 0; // Index to track the number of cars on the track

    // Read from standard input until EOF
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(car) = line.trim().parse::<i32>() {
            if car == 0 { // Exit command
                if i > 0 { // Ensure there is a car to exit
                    i -= 1; // Decrement the count of cars
                    println!("{}", railroad[i]); // Output the car number that exits
                }
                // If there are no cars, the behavior is not defined as per the spec
            } else { // Enter command
                if i < 10 { // Ensure we do not exceed the fixed size
                    railroad[i] = car; // Add the car to the track
                    i += 1; // Increment the count of cars
                }
                // If we receive more than 10 cars, we ignore the excess as per the spec
            }
        }
    }
}
