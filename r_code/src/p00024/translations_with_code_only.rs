use std::io::{self, BufRead};
use std::f32;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap(); // Unwrap to handle potential I/O errors
        let v: f32 = line.trim().parse().unwrap(); // Parse the input as f32

        let t = v / 9.8; // Calculate time
        let y = t * t * 4.9; // Calculate y

        // Calculate the ceiling of y / 5.0 and add 1, then print as integer
        let result = (y / 5.0).ceil() as i32 + 1;
        println!("{}", result);
    }
}
