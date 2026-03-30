use std::io::{self, BufRead};

fn main() {
    // Define the convergence criteria
    const EPS: f64 = 1e-7; // Precision for convergence

    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Handle each line of input
        let input = line.expect("Failed to read line");
        let number: i32 = match input.trim().parse() {
            Ok(num) if num >= 0 => num, // Ensure non-negative input
            Ok(-1) => break, // Termination condition
            _ => continue, // Ignore invalid input
        };

        // Compute the cube root using Newton's method
        let mut guess = number as f64; // Initial guess
        loop {
            let next_guess = guess - (guess.powi(3) - number as f64) / (3.0 * guess.powi(2));
            if (next_guess - guess).abs() < EPS {
                guess = next_guess; // Update guess if within precision
                break; // Converged
            }
            guess = next_guess; // Update guess for next iteration
        }

        // Output the result formatted to six decimal places
        println!("{:.6}", guess);
    }
}
