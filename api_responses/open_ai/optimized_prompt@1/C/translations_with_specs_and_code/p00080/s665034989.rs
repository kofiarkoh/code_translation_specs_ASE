use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.unwrap(); // Assuming valid integer input as per spec
        let q: i32 = input.trim().parse().unwrap(); // Parse input to integer

        // Termination condition
        if q == -1 {
            break;
        }

        // Compute the cube root using Newton's method
        let mut px = q as f64 / 2.0; // Initial guess
        let epsilon = 1e-5; // Convergence criterion

        loop {
            let x = px - (px * px * px - q as f64) / (3.0 * px * px);
            if (x * x * x - q as f64).abs() < epsilon * q as f64 {
                // If the result is within the acceptable range, print it
                println!("{:.6}", x);
                break;
            }
            px = x; // Update guess
        }
    }
}
