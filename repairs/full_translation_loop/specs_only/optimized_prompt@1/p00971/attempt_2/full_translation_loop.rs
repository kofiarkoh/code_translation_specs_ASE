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

    let len1 = s1.len();
    let len2 = s2.len();

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // Fill the dynamic programming table based on the rules
    for i in 1..=len1 {
        for j in 1..=len2 {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1; // Match case
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // No match, take max
            }
        }
    }

    // Construct the output sequence based on the dp table
    let mut output = String::new();
    let (mut i, mut j) = (len1, len2);

    while i > 0 && j > 0 {
        if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
            output.push(s1.chars().nth(i - 1).unwrap()); // Append matching character
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            output.push('0'); // Transition based on the dp table
            i -= 1;
        } else {
            output.push('1'); // Transition based on the dp table
            j -= 1;
        }
    }

    // If there are remaining characters in s1 or s2, append '0's or '1's
    while i > 0 {
        output.push('0');
        i -= 1;
    }
    while j > 0 {
        output.push('1');
        j -= 1;
    }

    // The output is constructed in reverse order, so we need to reverse it
    output = output.chars().rev().collect();

    // Print the output followed by a newline
    println!("{}", output);
}
