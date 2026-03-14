use std::io::{self, BufRead};

fn main() {
    // Define the epsilon for floating-point comparison
    const EPSILON: f64 = 1e-10;

    // Initialize counts for left and right values in the specified ranges
    let mut left_counts = [0; 4]; // Categories for left values
    let mut right_counts = [0; 4]; // Categories for right values

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Unwrap the line to get the input
        let line = line.unwrap();
        // Split the line into two floating-point values
        let values: Vec<&str> = line.split_whitespace().collect();
        
        // Ensure we have exactly two values
        if values.len() != 2 {
            continue; // Skip invalid lines
        }

        // Parse the left and right values
        let left_value: f64 = values[0].parse().unwrap();
        let right_value: f64 = values[1].parse().unwrap();

        // Update counts for left value
        update_counts(left_value, &mut left_counts);
        // Update counts for right value
        update_counts(right_value, &mut right_counts);
    }

    // Output the counts for left and right values
    println!("Left counts: {:?}", left_counts);
    println!("Right counts: {:?}", right_counts);
}

// Function to update counts based on the value ranges
fn update_counts(value: f64, counts: &mut [usize; 4]) {
    if value < -EPSILON {
        counts[0] += 1; // Category 1: value < 0
    } else if value >= -EPSILON && value <= EPSILON {
        counts[1] += 1; // Category 2: -epsilon <= value <= epsilon
    } else if value > EPSILON && value < 1.0 {
        counts[2] += 1; // Category 3: 0 < value < 1
    } else {
        counts[3] += 1; // Category 4: value >= 1
    }
}
