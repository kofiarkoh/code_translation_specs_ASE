use std::io;

fn longest_common_subsequence(s1: &str, s2: &str) -> String {
    let n = s1.len();
    let m = s2.len();
    
    // Create a 2D vector for dynamic programming table
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Fill the DP table
    for i in 1..=n {
        for j in 1..=m {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the LCS from the DP table
    let mut lcs = String::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
            lcs.push(s1.as_bytes()[i - 1] as char);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // The LCS is constructed in reverse order
    lcs.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    
    // Read input line
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Ensure there are exactly two strings
    if parts.len() != 2 {
        panic!("Expected two input strings");
    }
    
    let s1 = parts[0];
    let s2 = parts[1];
    
    // Ensure the strings do not exceed the maximum length
    if s1.len() > 2999 || s2.len() > 2999 {
        panic!("Input string exceeds maximum length");
    }

    // Compute and output the longest common subsequence
    let lcs = longest_common_subsequence(s1, s2);
    println!("{}", lcs);
}
