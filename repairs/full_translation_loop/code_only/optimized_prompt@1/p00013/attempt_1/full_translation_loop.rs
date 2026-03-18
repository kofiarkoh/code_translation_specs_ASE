use std::io::{self, BufRead};

fn main() {
    let mut railroad = [0; 10]; // Initialize an array of 10 integers
    let mut i = 0; // Index for the railroad array

    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let car: i32 = line.trim().parse().expect("Failed to parse integer");

        if car == 0 { // 出ていく (car is leaving)
            if i > 0 { // Ensure we don't go out of bounds
                i -= 1;
                println!("{}", railroad[i]); // Print the last car that is leaving
            }
        } else { // 入ってくる (car is entering)
            if i < 10 { // Ensure we don't go out of bounds
                railroad[i] = car; // Add the car to the railroad
                i += 1;
            }
        }
    }
}
