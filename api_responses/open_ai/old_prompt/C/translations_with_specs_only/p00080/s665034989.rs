use std::io::{self, Write};

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        print!("Enter a non-negative integer (or -1 to exit): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input as an integer
        let q: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and continue the loop
        };

        // Check for the sentinel value to exit
        if q == -1 {
            break;
        }

        // Ensure q is non-negative
        if q < 0 {
            continue; // Ignore negative inputs (though the spec assumes valid input)
        }

        // Calculate the cube root using the Newton-Raphson method
        let cube_root = newton_raphson_cube_root(q as f64);
        
        // Print the result formatted to six decimal places
        println!("{:.6}", cube_root);
    }
}

// Function to compute the cube root using the Newton-Raphson method
fn newton_raphson_cube_root(q: f64) -> f64 {
    // Initial guess for the cube root
    let mut x = q / 3.0; // Start with a guess (q/3 is a reasonable starting point)
    let tolerance = 1e-7; // Define a tolerance for convergence

    // Iterate until the result is sufficiently close to the actual cube root
    loop {
        let px = x; // Store the previous guess
        x = (2.0 * px + q / (px * px)) / 3.0; // Update the guess using Newton's method

        // Check for convergence
        if (x - px).abs() < tolerance {
            break; // Exit the loop if the guess is close enough
        }
    }

    x // Return the computed cube root
}
