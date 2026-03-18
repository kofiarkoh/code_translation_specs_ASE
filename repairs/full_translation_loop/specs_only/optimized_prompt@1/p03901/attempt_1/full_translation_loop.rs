use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Ensure we have exactly two parts: an integer and a double
    if parts.len() != 2 {
        eprintln!("Invalid input format. Expected an integer followed by a double.");
        return;
    }

    // Parse the position as an integer
    let position: i32 = match parts[0].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid integer value.");
            return;
        }
    };

    // Parse the probability rate as a double
    let probability_rate: f64 = match parts[1].parse() {
        Ok(value) if value >= 0.0 && value <= 100.0 => value,
        _ => {
            eprintln!("Probability rate must be between 0 and 100.");
            return;
        }
    };

    // Calculate the expected value
    let expected_value = (position as f64) * (probability_rate / 100.0);

    // Output the result formatted to seven decimal places
    println!("{:.7}", expected_value);
}
