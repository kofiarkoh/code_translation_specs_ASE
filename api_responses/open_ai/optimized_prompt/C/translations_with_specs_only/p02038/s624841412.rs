use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which should be the integer count
    let count_line = lines.next().unwrap().expect("Failed to read line");
    let count: usize = count_line.trim().parse().expect("Invalid integer input");

    // Check precondition: count must be >= 2
    if count < 2 {
        std::process::exit(1); // Terminate with non-zero exit code
    }

    // Read the boolean characters
    let mut boolean_values = Vec::new();
    for _ in 0..count {
        let char_line = lines.next().unwrap().expect("Failed to read line");
        let ch = char_line.trim().chars().next().expect("No character found");

        // Check precondition: character must be 'T' or 'F'
        if ch != 'T' && ch != 'F' {
            std::process::exit(1); // Terminate with non-zero exit code
        }
        boolean_values.push(ch);
    }

    // Evaluate the logical expression
    let result = evaluate_logical_expression(&boolean_values);

    // Output the result
    println!("{}", result);
}

// Function to evaluate the logical expression based on the input characters
fn evaluate_logical_expression(values: &[char]) -> char {
    // Initialize the result with the first boolean value
    let mut result = values[0];

    // Iterate through the boolean values and apply logical AND operation
    for &value in &values[1..] {
        // If any value is 'F', the result is 'F'
        if value == 'F' {
            return 'F';
        }
    }
    // If all values are 'T', the result is 'T'
    'T'
}
