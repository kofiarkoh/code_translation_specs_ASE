use std::io::{self, BufRead};

const MOD: usize = 1_000_000_007;

fn count_distinct_subsequences(t: &str, b: &str) -> usize {
    let n = t.len();
    let m = b.len();

    // Handle the case where either string is empty
    if n == 0 || m == 0 {
        return 1; // The empty string is a subsequence of any string
    }

    // Create a 2D DP table initialized to 0
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: An empty `b` is a subsequence of any prefix of `t`
    for i in 0..=n {
        dp[i][0] = 1; // There's one way to form an empty subsequence
    }

    // Fill the DP table
    for i in 1..=n {
        for j in 1..=m {
            // If characters match, we can either use this character or not
            if t.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD;
            } else {
                // If they don't match, we can't use this character from `t`
                dp[i][j] = dp[i - 1][j] % MOD;
            }
        }
    }

    // The result is in the bottom-right cell of the DP table
    dp[n][m]
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read input strings `t` and `b`
    let mut lines = handle.lines();
    let t = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    // Check for length constraints
    if t.len() > 1000 || b.len() > 1000 {
        panic!("Input strings exceed maximum length of 1000 characters.");
    }

    // Compute the number of distinct subsequences
    let result = count_distinct_subsequences(&t, &b);

    // Output the result
    println!("{}", result);
}
