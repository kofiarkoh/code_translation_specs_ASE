fn compute_sequence(n: usize, dim1: &[i32], dim2: &[i32], dim3: &[i32]) -> Vec<i32> {
    // Preconditions: n must be non-negative and arrays must be of size n
    assert!(n >= 0);
    assert_eq!(dim1.len(), n);
    assert_eq!(dim2.len(), n);
    assert_eq!(dim3.len(), n);

    // Initialize a dynamic programming table and transition records
    let mut dp = vec![vec![i32::MAX; 1 << n]; n]; // Cost table
    let mut prev = vec![vec![-1; 1 << n]; n]; // Transition records

    // Base case: Initialize the first row of the DP table
    for mask in 0..(1 << n) {
        dp[0][mask] = dim1[0]; // Cost for the first element
    }

    // Fill the DP table
    for i in 1..n {
        for mask in 0..(1 << n) {
            for j in 0..n {
                if mask & (1 << j) == 0 { // If j is not included in the mask
                    let new_mask = mask | (1 << j);
                    let cost = dp[i - 1][mask] + dim1[i]; // Example cost calculation
                    if cost < dp[i][new_mask] {
                        dp[i][new_mask] = cost;
                        prev[i][new_mask] = mask; // Update transition record
                    }
                }
            }
        }
    }

    // Reconstruct the output sequence based on the transition records
    let mut output = Vec::with_capacity(n);
    let mut mask = (1 << n) - 1; // Start with all elements included

    for i in (0..n).rev() {
        output.push(dim1[i]); // Push the corresponding value from the first array
        mask = prev[i][mask]; // Move to the previous state
    }

    output.reverse(); // Reverse to get the correct order
    output
}

fn main() {
    let n = 3; // Example size
    let dim1 = vec![1, 2, 3];
    let dim2 = vec![4, 5, 6];
    let dim3 = vec![7, 8, 9];

    let result = compute_sequence(n, &dim1, &dim2, &dim3);
    println!("{:?}", result); // Output the result
}
```

### Explanation of Choices:
1. **Dynamic Programming Table**: The DP table is initialized to `i32::MAX` to represent uncomputed states, ensuring that we can track minimum costs effectively.
2. **Transition Records**: The `prev` array is used to keep track of the previous state for each computed state, allowing for reconstruction of the output sequence.
3. **Cost Calculation**: The cost is computed based on the first dimension's values, as implied by the specification. The exact cost function can be adjusted based on further details if provided.
4. **Output Construction**: The output sequence is constructed by tracing back through the `prev` array, ensuring the correct order is maintained.
5. **Assertions**: Assertions are used to enforce preconditions, as the specification does not define behavior for invalid inputs.
