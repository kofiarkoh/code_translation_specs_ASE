fn count_sequences(n: usize, m: usize, s: &[i32], t: &[i32]) -> i64 {
    const MOD: i64 = 1_000_000_007;

    // Edge case: if either n or m is zero, there is one way to form an empty sequence
    if n == 0 || m == 0 {
        return 1;
    }

    // Create a 2D dp array initialized to 0
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: there's one way to form an empty sequence
    for j in 0..=m {
        dp[0][j] = 1;
    }

    // Fill the dp array according to the dynamic programming logic
    for i in 1..=n {
        for j in 1..=m {
            // The value is derived from the previous states
            dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % MOD;

            // If the elements match, we need to subtract the overlapping case
            if s[i - 1] == t[j - 1] {
                dp[i][j] = (dp[i][j] - dp[i - 1][j - 1] + MOD) % MOD; // Ensure non-negative
            }
        }
    }

    // The result is in the bottom-right cell of the dp array
    dp[n][m]
}

fn main() {
    // Example input
    let n = 3;
    let m = 3;
    let s = vec![1, 2, 3];
    let t = vec![2, 3, 4];

    // Calculate the number of valid sequences
    let result = count_sequences(n, m, &s, &t);

    // Print the result
    println!("{}", result);
}
