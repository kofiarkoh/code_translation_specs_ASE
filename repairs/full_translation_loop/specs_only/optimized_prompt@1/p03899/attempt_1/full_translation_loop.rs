use std::io::{self, Write};

fn compute_max_weighted_sum(n: usize, m: usize, k: usize, sequence: Vec<i32>) -> i32 {
    // Create a dynamic programming array to store maximum sums
    let mut dp = vec![0; n + 1];

    // Iterate through the number of iterations
    for _ in 0..k {
        let mut new_dp = dp.clone(); // Create a new DP array for this iteration

        // Update the DP array based on the constraints
        for i in 0..n {
            // Calculate the maximum sum considering the gap constraint
            let start_index = if i >= m { i - m } else { 0 };
            let max_prev = (start_index..=i).map(|j| dp[j]).max().unwrap_or(0);
            new_dp[i + 1] = max_prev + sequence[i];
        }

        dp = new_dp; // Move to the next iteration
    }

    // The result is the maximum value in the last DP array
    *dp.iter().max().unwrap_or(&0)
}

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let n = parts[0];
    let m = parts[1];
    let k = parts[2];

    // Read the sequence of integers
    let mut sequence = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    sequence.extend(input.split_whitespace()
        .map(|s| s.parse().expect("Parse error")));

    // Ensure the sequence length matches n
    assert_eq!(sequence.len(), n);

    // Compute the maximum weighted sum
    let result = compute_max_weighted_sum(n, m, k, sequence);

    // Print the result
    println!("{}", result);
}
