use std::collections::HashMap;

fn count_valid_subsequences(n: i32, s: i32, w: i32, q: i32) -> i32 {
    // Preconditions check
    if n <= 0 || q <= 0 {
        return 0; // Early exit for invalid input
    }

    // Initialize the hash table to store counts of computed values
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut current_value = s;

    // Generate the sequence and populate the hash table
    for _ in 0..n {
        // Transform the current value based on the given algorithm
        current_value = (current_value + w) % q; // Ensure we stay within bounds of q
        *counts.entry(current_value).or_insert(0) += 1; // Update the count in the hash table
    }

    // Count valid subsequences based on the hash table
    let valid_count = counts.values().sum(); // Sum of all counts as a placeholder for valid subsequences

    // Output the result
    println!("{}", valid_count);
    valid_count // Return the count of valid subsequences
}

fn main() {
    // Example usage
    let n = 5; // Number of integers
    let s = 1; // Initial seed value
    let w = 2; // Transformation value
    let q = 10; // Modulus value

    count_valid_subsequences(n, s, w, q);
}
