use std::fmt;

fn main() {
    // Example input values for testing
    let position: i32 = 5; // Example position (can be even or odd)
    let probability_rate: f64 = 75.0; // Example probability rate in percentage

    // Validate the probability rate
    if probability_rate < 0.0 || probability_rate > 100.0 {
        panic!("Probability rate must be in the range [0, 100]");
    }

    // Calculate the expected value
    let expected_value = calculate_expected_value(position, probability_rate / 100.0);
    
    // Output the result formatted to seven decimal places
    println!("{:.7}", expected_value);
}

// Recursive function to calculate the expected value
fn calculate_expected_value(position: i32, probability_rate: f64) -> f64 {
    if position % 2 == 0 {
        // For even positions, return position divided by the probability rate
        position as f64 / probability_rate
    } else {
        // For odd positions, calculate using adjacent even positions
        let even_before = calculate_expected_value(position - 1, probability_rate);
        let even_after = calculate_expected_value(position + 1, probability_rate);
        
        // Calculate expected value for odd position
        (even_before * probability_rate) + (even_after * (1.0 - probability_rate))
    }
}
