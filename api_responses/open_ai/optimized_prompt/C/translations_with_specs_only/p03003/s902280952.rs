fn count_distinct_sequences(n: usize, m: usize, s: &[i32], t: &[i32]) -> i32 {
    const MOD: i32 = 1_000_000_007;

    // Preconditions check: n and m must be non-negative and arrays must have correct lengths
    if n != s.len() || m != t.len() {
        panic!("Input arrays must have lengths equal to n and m respectively.");
    }

    // Initialize a 2D dp array with dimensions (n+1) x (m+1)
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: There's one way to form an empty sequence
    dp[0][0] = 1;

    // Fill the dp array according to the state transitions
    for i in 0..=n {
        for j in 0..=m {
            // If we can extend the sequence from s
            if i > 0 {
                dp[i][j] = (dp[i][j] + dp[i - 1][j]) % MOD;
            }
            // If we can extend the sequence from t
            if j > 0 {
                dp[i][j] = (dp[i][j] + dp[i][j - 1]) % MOD;
            }
            // If both sequences can be extended, we need to subtract the overlap
            if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                dp[i][j] = (dp[i][j] - dp[i - 1][j - 1] + MOD) % MOD; // Ensure non-negative
            }
        }
    }

    // The result is the number of distinct sequences of length n from s and length m from t
    dp[n][m]
}

fn main() {
    // Example usage
    let n = 3;
    let m = 2;
    let s = [1, 2, 3];
    let t = [2, 3];
    
    let result = count_distinct_sequences(n, m, &s, &t);
    println!("{}", result); // Output the result
}
