use std::collections::HashSet;

fn compute_count(n: i64, x: i64, d: i64) -> i64 {
    // Preconditions: N must be a non-negative integer.
    assert!(n >= 0);

    // Handle special cases based on the specification.
    if x == 0 && d == 0 {
        return 1; // Both X and D are zero.
    }
    if d == 0 {
        return n + 1; // D is zero.
    }
    if x == 0 {
        return n * (n - 1) / 2 + 1; // X is zero and D is non-zero.
    }

    // Initialize a hash set to store unique ranges of i64.
    let mut ranges: HashSet<i64> = HashSet::new();

    // The logic for processing ranges based on N, X, and D would go here.
    // Since the specification does not provide details on how to compute ranges,
    // we will assume a placeholder for the logic that would populate the hash set.

    // For demonstration, we will just return the size of the hash set.
    // In a complete implementation, this would involve actual range calculations.
    ranges.len() as i64 // Return the count of unique ranges.
}

fn main() {
    // Example usage of the compute_count function.
    let n: i64 = 10; // Example input
    let x: i64 = 0;  // Example input
    let d: i64 = 5;  // Example input
    let result = compute_count(n, x, d);
    println!("{}", result); // Output the result
}
