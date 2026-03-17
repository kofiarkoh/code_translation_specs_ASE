fn compute_value(n: usize, sequence: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007; // Define the modulo constant
    let mut cumulative_xor: i32 = 0; // Initialize cumulative XOR
    let mut zero_count: i32 = 0; // Initialize count of occurrences where cumulative XOR is zero

    // Iterate through the sequence
    for &num in &sequence {
        cumulative_xor ^= num; // Update cumulative XOR with the current number

        // Check if cumulative XOR is zero
        if cumulative_xor == 0 {
            zero_count += 1; // Increment the count if cumulative XOR is zero
        }
    }

    // Return the result as the count of zero occurrences modulo 10^9 + 7
    (zero_count % MOD + MOD) % MOD // Ensure non-negativity
}

fn main() {
    // Example usage
    let n = 5; // Number of elements
    let sequence = vec![1, 2, 3, 4, 5]; // Example sequence
    let result = compute_value(n, sequence); // Compute the result
    println!("{}", result); // Output the result
}
