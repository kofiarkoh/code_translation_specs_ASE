use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read exactly ten integers from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .take(10) // Ensure we only read 10 lines
        .filter_map(|line| line.ok()?.parse().ok()) // Parse integers, ignore invalid lines
        .collect();

    // Check if we have exactly 10 integers
    if input.len() != 10 {
        // Undefined behavior as per specification; we do nothing
        return;
    }

    // Use a HashSet to store unique values
    let mut unique_values: HashSet<i32> = HashSet::new();
    for &value in &input {
        unique_values.insert(value);
    }

    // Collect unique values and sort them in descending order
    let mut sorted_unique: Vec<i32> = unique_values.into_iter().collect();
    sorted_unique.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

    // Prepare the output array
    let mut output = [0; 3];

    // Fill the output with the top three unique values or zeros
    for (i, &value) in sorted_unique.iter().take(3).enumerate() {
        output[i] = value;
    }

    // Print the output
    println!("{}, {}, {}", output[0], output[1], output[2]);
}
