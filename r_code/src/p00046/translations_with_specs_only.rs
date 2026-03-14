use std::io::{self, BufRead};

fn main() {
    // Create a vector to store the input floating-point numbers
    let mut numbers: Vec<f64> = Vec::new();

    // Read from standard input until EOF
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Parse each line as a floating-point number
        if let Ok(value) = line {
            if let Ok(num) = value.trim().parse::<f64>() {
                numbers.push(num);
            }
        }
    }

    // Preconditions state that at least one number must be provided
    if numbers.is_empty() {
        // Undefined behavior as per specification; we do nothing
        return;
    }

    // Calculate the maximum and minimum values
    let max_value = *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_value = *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    // Calculate the difference
    let difference = max_value - min_value;

    // Output the result formatted to one decimal place
    println!("{:.1}", difference);
}
