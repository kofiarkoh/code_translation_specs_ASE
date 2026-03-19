fn minimum_cost(n: usize, m: usize, seq_a: &[i32], seq_b: &[i32]) -> i32 {
    // Preconditions check
    assert!(n >= 1, "n must be >= 1");
    assert!(m >= 1, "m must be >= 1");
    assert!(seq_a.len() == n, "Length of seq_a must be n");
    assert!(seq_b.len() == m, "Length of seq_b must be m");

    // Initialize the cost matrix with a high value
    let mut cost_matrix = vec![vec![i32::MAX; m]; n];

    // Set the first column to zero as per the invariant
    for i in 0..n {
        cost_matrix[i][0] = 0;
    }

    // Compute the cost matrix based on the product of elements and their indices
    for i in 0..n {
        for j in 0..m {
            // Calculate the cost based on the product of the elements and their indices
            let cost = seq_a[i] * seq_b[j] * (i as i32 + j as i32);
            // Update the cost matrix with the minimum cost found
            if cost < cost_matrix[i][j] {
                cost_matrix[i][j] = cost;
            }
        }
    }

    // Return the minimum cost associated with the last elements of both sequences
    // This assumes we want the minimum cost from the last row and last column
    let min_cost = cost_matrix[n - 1][m - 1];
    min_cost
}
