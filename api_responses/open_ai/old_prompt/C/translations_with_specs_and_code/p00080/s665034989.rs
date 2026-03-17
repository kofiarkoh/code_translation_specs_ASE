use std::io::{self, Write};

const EPS: f64 = 1e-5;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        // Read input from stdin
        if stdin.read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        // Parse the input as an integer
        let q: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        // Check for sentinel value
        if q == -1 {
            break;
        }

        // Ensure q is non-negative
        if q < 0 {
            continue; // Skip negative values
        }

        // Newton-Raphson method for cube root calculation
        let mut px = q as f64 / 2.0;
        let mut x;

        loop {
            x = px - (px * px * px - q as f64) / (3.0 * px * px);
            if (x * x * x - q as f64).abs() < EPS * q as f64 {
                break; // Convergence condition
            }
            px = x; // Update px for the next iteration
        }

        // Print the result formatted to six decimal places
        println!("{:.6}", x);
    }
}
