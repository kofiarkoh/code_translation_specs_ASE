fn max_value(n: usize, x: usize, y: usize, items: &[(usize, usize, usize, usize)]) -> usize {
    // Create a dynamic programming table initialized to 0
    let mut dp = vec![vec![0; y + 1]; x + 1];

    // Iterate over each item
    for &(a, b, c, d) in items.iter() {
        // Traverse the DP table in reverse to avoid overwriting states we still need to use
        for i in (a..=x).rev() {
            for j in (0..=y).rev() {
                // Calculate the new resource level for the second category
                let new_y = j + d;
                // Ensure we do not exceed the maximum limit for the second category
                let capped_y = new_y.min(y);
                // Update the DP table with the maximum value obtainable
                dp[i][capped_y] = dp[i][capped_y].max(dp[i - a][j] + c);
            }
        }
    }

    // The maximum value obtainable is the maximum in the last row of the DP table
    dp.iter().flat_map(|row| row.iter()).cloned().max().unwrap_or(0)
}

// Example usage
fn main() {
    let n = 3;
    let x = 10;
    let y = 5;
    let items = vec![(2, 1, 3, 4), (1, 2, 2, 3), (3, 1, 1, 5)];
    let result = max_value(n, x, y, &items);
    println!("{}", result); // Output the result directly
}
