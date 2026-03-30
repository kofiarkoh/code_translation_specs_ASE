use std::io::{self, BufRead};

const EPS: f64 = 1e-5;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let q: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };

        if q == -1 {
            break; // Termination condition
        }

        if q < 0 {
            continue; // Ensure non-negative input
        }

        let mut px = q as f64 / 2.0;
        let mut x;

        loop {
            x = px - (px * px * px - q as f64) / (3.0 * px * px);
            if (x * x * x - q as f64).abs() < EPS * q as f64 {
                break; // Convergence condition
            }
            px = x;
        }

        println!("{:.6}", x); // Output formatted to six decimal places
    }
}
