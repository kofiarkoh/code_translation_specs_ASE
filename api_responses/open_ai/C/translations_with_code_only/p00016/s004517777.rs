use std::f64::consts::PI;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut d = 0.0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');

        // Parse mov and dir from the input
        let mov: f64 = match parts.next().unwrap().trim().parse() {
            Ok(value) => value,
            Err(_) => continue, // Skip invalid input
        };
        let dir: f64 = match parts.next().unwrap().trim().parse() {
            Ok(value) => value,
            Err(_) => continue, // Skip invalid input
        };

        // Check for termination condition
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // Update x and y based on the current direction and movement
        x += (d * PI / 180.0).sin() * mov;
        y += (d * PI / 180.0).cos() * mov;

        // Update the direction
        d += dir;
    }

    // Print the results as integers
    println!("{}\n{}", x as i32, y as i32);
}
