use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Check if we have exactly two parts
    if parts.len() != 2 {
        // Undefined behavior as per specification if input is invalid
        return;
    }

    // Parse the position as an integer
    let position: i32 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return, // Undefined behavior if parsing fails
    };

    // Parse the probability rate as a double
    let probability_rate: f64 = match parts[1].parse() {
        Ok(rate) if rate >= 0.0 && rate <= 100.0 => rate,
        _ => return, // Undefined behavior if parsing fails or rate is out of bounds
    };

    // Calculate the expected value
    let expected_value = (position as f64) * (probability_rate / 100.0);

    // Output the result formatted to seven decimal places
    println!("{:.7}", expected_value);
}
