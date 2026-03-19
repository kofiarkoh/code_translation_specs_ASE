fn count_interleavings(len_a: usize, len_b: usize, seq_a: &[i32], seq_b: &[i32], modulo: i32) -> i32 {
    // Create a dynamic programming table with (len_a + 1) x (len_b + 1) dimensions
    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

    // Base case: There's one way to interleave two empty sequences
    dp[0][0] = 1;

    // Fill the dynamic programming table
    for i in 0..=len_a {
        for j in 0..=len_b {
            // If we can take from seq_a
            if i > 0 {
                dp[i][j] = (dp[i][j] + dp[i - 1][j]) % modulo;
            }
            // If we can take from seq_b
            if j > 0 {
                dp[i][j] = (dp[i][j] + dp[i][j - 1]) % modulo;
            }
        }
    }

    // The result is in the bottom-right cell of the table
    dp[len_a][len_b]
}

// Example usage
fn main() {
    let seq_a = vec![1, 2, 3];
    let seq_b = vec![2, 3, 4];
    let len_a = seq_a.len();
    let len_b = seq_b.len();
    let modulo = 1_000_000_007; // Example modulo constant

    let result = count_interleavings(len_a, len_b, &seq_a, &seq_b, modulo);
    println!("{}", result); // Output the number of distinct interleavings
}
