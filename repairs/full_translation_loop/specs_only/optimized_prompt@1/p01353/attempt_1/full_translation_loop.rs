use std::io::{self, BufRead};

fn main() {
    // Read all input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the count of game parameters
    let count_line = lines.next().unwrap().expect("Failed to read line");
    let count: usize = count_line.trim().parse().expect("Invalid count");

    // Initialize a vector to hold the game parameters
    let mut parameters = Vec::new();

    // Read the specified number of parameter sets
    for _ in 0..count {
        let param_line = lines.next().unwrap().expect("Failed to read line");
        let params: Vec<i64> = param_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid parameter"))
            .collect();
        parameters.push(params);
    }

    // Process the parameters to calculate the result
    let result = calculate_result(&parameters);

    // Output the result or an error indicator
    match result {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Error"),
    }
}

// Function to calculate the result based on the game parameters
fn calculate_result(parameters: &[Vec<i64>]) -> Result<i64, ()> {
    let mut total = 0;

    // Example processing logic (this should be replaced with actual game logic)
    for param_set in parameters {
        // Check if the parameter set meets certain conditions
        if param_set.is_empty() || param_set.len() > 10 {
            return Err(()); // Error if the parameter set is empty or too large
        }

        // Example calculation (sum of parameters)
        let sum: i64 = param_set.iter().sum();

        // Check for overflow (this is a simple check; more complex logic may be needed)
        if total.checked_add(sum).is_none() {
            return Err(()); // Error if overflow occurs
        }

        total += sum;
    }

    // Return the calculated total
    Ok(total)
}
