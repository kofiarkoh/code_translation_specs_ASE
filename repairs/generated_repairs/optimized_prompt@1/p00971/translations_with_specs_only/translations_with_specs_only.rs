use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the two binary strings
    let s1 = lines.next().unwrap().unwrap();
    let s2 = lines.next().unwrap().unwrap();

    // Ensure the input strings do not exceed 4000 characters
    assert!(s1.len() <= 4000 && s2.len() <= 4000);
    assert!(s1.chars().all(|c| c == '0' || c == '1'));
    assert!(s2.chars().all(|c| c == '0' || c == '1'));

    // Initialize the dynamic programming table
    let n = s1.len();
    let m = s2.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Fill the dynamic programming table based on the rules
    for i in 1..=n {
        for j in 1..=m {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1; // Match case
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // No match, take max
            }
        }
    }

    // Backtrack to find the optimal path
    let mut result = String::new();
    let mut i = n;
    let mut j = m;

    while i > 0 && j > 0 {
        if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
            result.push(s1.chars().nth(i - 1).unwrap()); // Add matching character
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1; // Move up in the table
        } else {
            j -= 1; // Move left in the table
        }
    }

    // The result is built in reverse order, so we need to reverse it
    result = result.chars().rev().collect();

    // Output the result, ensuring to match the expected output format
    let output = format!("{:0width$b}", u32::from_str_radix(&result, 2).unwrap_or(0), width = std::cmp::max(n, m));
    println!("{}", output);
}
