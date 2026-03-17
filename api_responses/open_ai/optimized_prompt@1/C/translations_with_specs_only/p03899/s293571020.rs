fn main() {
    // Example input values; in a real scenario, these would be read from standard input.
    let n = 5; // Number of elements in the sequence
    let m = 2; // Gap constraint
    let k = 3; // Number of iterations
    let sequence = vec![1, 2, 3, 4, 5]; // The sequence of integers

    // Call the function to compute the maximum weighted sum
    let result = max_weighted_sum(n, m, k, sequence);
    
    // Print the result
    println!("{}", result);
}

fn max_weighted_sum(n: usize, m: usize, k: usize, sequence: Vec<i32>) -> i32 {
    // Preconditions: n, m, k must be non-negative and sequence must have exactly n elements
    assert!(n >= 0);
    assert!(m >= 0);
    assert!(k >= 0);
    assert!(sequence.len() == n);

    // Dynamic programming array to store maximum sums
    let mut dp = vec![0; n];

    // Fill the dynamic programming array
    for _ in 0..k {
        let mut new_dp = dp.clone(); // Create a new DP array for the current iteration
        for i in 0..n {
            // Calculate the maximum weighted sum considering the gap constraint
            let mut max_sum = 0;
            for j in (0..i).rev() {
                if i as isize - j as isize > m as isize {
                    break; // Respect the gap constraint
                }
                max_sum = max_sum.max(dp[j]);
            }
            new_dp[i] = max_sum + sequence[i]; // Update the new DP value
        }
        dp = new_dp; // Move to the next iteration
    }

    // The result is the maximum value in the DP array
    *dp.iter().max().unwrap_or(&0) // Return the maximum computed weighted sum
}
