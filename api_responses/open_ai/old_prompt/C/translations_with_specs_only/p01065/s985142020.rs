fn main() {
    // Read input values
    let (n, x, y, items) = read_input();

    // Initialize the dynamic programming table
    let mut dp = vec![vec![-1; (y + 1) as usize]; (x + 1) as usize];
    dp[0][0] = 0; // Base case: no resources used, no value gained

    // Process each item
    for (a, b, c, d) in items {
        // Iterate backwards to avoid overwriting states in the current iteration
        for i in (0..=x).rev() {
            for j in (0..=y).rev() {
                if dp[i as usize][j as usize] != -1 {
                    let new_i = i.saturating_add(a);
                    let new_j = j.saturating_add(b);
                    if new_i <= x && new_j <= y {
                        dp[new_i as usize][new_j as usize] = 
                            dp[new_i as usize][new_j as usize].max(dp[i as usize][j as usize] + d);
                    }
                }
            }
        }
    }

    // Find the maximum value obtainable
    let max_value = dp.iter().flat_map(|row| row.iter()).filter_map(|&v| if v != -1 { Some(v) } else { None }).max().unwrap_or(0);

    // Output the result
    println!("{}", max_value);
}

// Function to read input values
fn read_input() -> (usize, usize, usize, Vec<(usize, usize, usize, usize)>) {
    // Placeholder for reading input; replace with actual input reading logic
    // For example, you could use std::io::stdin() to read from standard input
    // Here we return dummy values for demonstration purposes
    (3, 10, 10, vec![(2, 3, 5, 10), (3, 4, 6, 15), (5, 2, 4, 20)])
}
