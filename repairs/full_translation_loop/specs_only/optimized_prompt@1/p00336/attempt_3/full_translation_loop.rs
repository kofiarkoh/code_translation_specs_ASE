use std::io::{self, BufRead};

const MOD: u64 = 1_000_000_007; // Specified large prime number

fn count_distinct_subsequences(target: &str, source: &str) -> u64 {
    let n = target.len();
    let m = source.len();
    
    // Create a 2D array to store the counts of distinct subsequences
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: An empty target string has one subsequence (the empty subsequence)
    for j in 0..=m {
        dp[0][j] = 1;
    }

    // Fill the dp array
    for i in 1..=n {
        for j in 1..=m {
            // If characters match, we can either include or exclude the character
            if target.as_bytes()[i - 1] == source.as_bytes()[j - 1] {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i][j - 1]) % MOD;
            } else {
                // If they don't match, we can only exclude the character from source
                dp[i][j] = dp[i][j - 1] % MOD;
            }
        }
    }

    // The result is the count of distinct subsequences of target in source
    dp[n][m]
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the target and source strings
    let target = lines.next().unwrap().unwrap();
    let source = lines.next().unwrap().unwrap();

    // Check for length constraints
    if target.len() > 1000 || source.len() > 1000 {
        // Behavior is not defined for inputs exceeding maximum length
        return;
    }

    // Calculate the number of distinct subsequences
    let result = count_distinct_subsequences(&target, &source);

    // Output the result
    println!("{}", result);
}
