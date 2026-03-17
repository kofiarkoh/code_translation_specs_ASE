use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the position (piPos)
    let pi_pos: i32 = handle.lines().next().unwrap().unwrap().parse().unwrap();
    
    // Read the probability rate (sdRate) and convert it to a fraction
    let sd_rate_percentage: f64 = handle.lines().next().unwrap().unwrap().parse().unwrap();
    let sd_rate = sd_rate_percentage / 100.0;

    // Ensure sdRate is within the valid range [0.0, 1.0]
    // This is a precondition check based on the specification
    assert!(sd_rate >= 0.0 && sd_rate <= 1.0, "sdRate must be between 0.0 and 1.0");

    // Calculate the expected value
    let expected_value = f_get_val(pi_pos, sd_rate);

    // Output the expected value formatted to seven decimal places
    println!("{:.7}", expected_value);
}

// Recursive function to calculate the expected value
fn f_get_val(pi_pos: i32, sd_rate: f64) -> f64 {
    // Base case: if pi_pos is 0, return 1.0 (as an example)
    if pi_pos == 0 {
        return 1.0;
    }
    // Recursive case: calculate expected value based on the position and probability rate
    // This is a placeholder for the actual recursive logic
    // The actual logic should be defined based on the specific requirements
    let expected_value = f_get_val(pi_pos - 1, sd_rate) * sd_rate + f_get_val(pi_pos - 1, sd_rate) * (1.0 - sd_rate);
    
    expected_value
}
