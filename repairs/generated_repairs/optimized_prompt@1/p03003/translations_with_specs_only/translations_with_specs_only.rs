// This program computes the number of distinct interleavings of two sequences
// while preserving the order of elements in each sequence.

fn count_interleavings(len_a: usize, len_b: usize, seq_a: &[i32], seq_b: &[i32], modulo: i32) -> i32 {
    // Preconditions: lengths are non-negative integers
    // Ensure the lengths match the provided sequences
    assert_eq!(len_a, seq_a.len());
    assert_eq!(len_b, seq_b.len());

    // Create a dynamic programming table to store the counts of interleavings
    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

    // Base case: there's one way to interleave two empty sequences
    dp[0][0] = 1;

    // Fill the dynamic programming table
    for i in 0..=len_a {
        for j in 0..=len_b {
            // If we can take an element from seq_a
            if i > 0 {
                dp[i][j] = (dp[i][j] + dp[i - 1][j]) % modulo;
            }
            // If we can take an element from seq_b
            if j > 0 {
                dp[i][j] = (dp[i][j] + dp[i][j - 1]) % modulo;
            }
        }
    }

    // The result is the number of interleavings of the full sequences
    dp[len_a][len_b]
}

// Example usage
fn main() {
    let seq_a = vec![1, 2, 3];
    let seq_b = vec![2, 3, 4];
    let modulo = 1_000_000_007; // A common large prime for modulo operations
    let result = count_interleavings(seq_a.len(), seq_b.len(), &seq_a, &seq_b, modulo);
    println!("{}", result);
}
