use std::io::{self, BufRead};

fn main() {
    // Read input strings from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first string
    let s1 = lines.next().unwrap().unwrap();
    // Read the second string
    let s2 = lines.next().unwrap().unwrap();

    // Lengths of the input strings
    let n = s1.len();
    let m = s2.len();

    // Create a 2D vector to store the counts of distinct subsequences
    // dp[i][j] will hold the count of distinct subsequences of s1[0..i] matching s2[0..j]
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: An empty s2 has one subsequence in any prefix of s1 (the empty subsequence)
    for i in 0..=n {
        dp[i][0] = 1;
    }

    // Fill the dp table
    for i in 1..=n {
        for j in 1..=m {
            // If characters match, we can either include the character or not
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % 1_000_000_007;
            } else {
                // If characters do not match, we can only exclude the character from s1
                dp[i][j] = dp[i - 1][j] % 1_000_000_007;
            }
        }
    }

    // The result is in dp[n][m], which is the count of distinct subsequences of s1 matching s2
    println!("{}", dp[n][m]);
}
