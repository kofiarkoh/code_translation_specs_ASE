// Define the modulo constant as specified in the postconditions
const MOD: i32 = 1_000_000_007;

// Function to compute the result based on the input sequence
fn compute_value(n: usize, sequence: Vec<i32>) -> i32 {
    // Preconditions: N must be non-negative and sequence must contain N integers
    assert!(n >= 0, "N must be a non-negative integer");
    assert_eq!(sequence.len(), n, "Sequence must contain N integers");

    // Initialize counters and intermediate storage
    let mut count = 0; // To count occurrences of a certain condition
    let mut result = 0; // Final result to be computed

    // Iterate through the sequence
    for &value in &sequence {
        // Example condition: count how many numbers are even
        if value % 2 == 0 {
            count += 1;
        }
        
        // Update result based on the count, using modular arithmetic
        result = (result + count) % MOD;
    }

    // Postcondition: The output value is guaranteed to be a non-negative integer
    result
}

// Main function to demonstrate usage
fn main() {
    // Example usage with a hardcoded sequence
    let n = 5; // Number of elements
    let sequence = vec![1, 2, 3, 4, 5]; // Input sequence

    // Compute the result
    let result = compute_value(n, sequence);

    // Output the result
    println!("{}", result);
}
