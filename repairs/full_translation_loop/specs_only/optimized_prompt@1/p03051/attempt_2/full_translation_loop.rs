fn compute_value(n: usize, sequence: &[i32]) -> i32 {
    const MOD: i32 = 1_000_000_007;

    // Ensure the input is valid as per the specification
    assert!(n >= 0, "N must be a non-negative integer");
    assert_eq!(sequence.len(), n, "Sequence must contain N integers");

    // Initialize counters and intermediate storage
    let mut count_even = 0;
    let mut count_odd = 0;

    // Process each element in the sequence
    for &value in sequence.iter() {
        // Count occurrences of even and odd numbers
        if value % 2 == 0 {
            count_even += 1;
        } else {
            count_odd += 1;
        }
    }

    // The final result is based on the counts of even and odd numbers
    // Here we assume the result is the difference between counts
    let result = (count_even - count_odd).rem_euclid(MOD) % MOD;

    // Ensure the result is non-negative as per postconditions
    result
}

fn main() {
    // Example usage
    let n = 5;
    let sequence = vec![1, 2, 3, 4, 5];
    let result = compute_value(n, &sequence);
    println!("{}", result);
}
