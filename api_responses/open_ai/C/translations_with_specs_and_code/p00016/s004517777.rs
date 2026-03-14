use std::f64::consts::PI;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut d = 0.0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let mov: f64 = match parts[0].trim().parse() {
            Ok(value) => value,
            Err(_) => continue, // Skip invalid movement input
        };

        let dir: f64 = match parts[1].trim().parse() {
            Ok(value) => value,
            Err(_) => continue, // Skip invalid direction input
        };

        // Termination condition
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // Update coordinates based on current direction and movement
        x += (d * PI / 180.0).sin() * mov;
        y += (d * PI / 180.0).cos() * mov;

        // Update direction
        d += dir;
    }

    // Output the final coordinates, rounded down to the nearest integer
    println!("{}", x.floor() as i32);
    println!("{}", y.floor() as i32);
}
