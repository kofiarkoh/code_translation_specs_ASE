use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        // Read input from standard input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse the input as an integer
        let q: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and continue prompting
        };

        // Check for termination signal
        if q == -1 {
            break;
        }

        // Initial guess for the cube root
        let mut x = q as f64 / 2.0;

        // Iteratively refine the guess using Newton's method
        while !(f64::abs(x * x * x - q as f64) < 0.00001 * q as f64) {
            x = x - (x * x * x - q as f64) / (3.0 * x * x);
        }

        // Output the computed cube root
        println!("{:.6}", x);
    }
}
