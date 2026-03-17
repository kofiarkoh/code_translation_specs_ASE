use std::io::{self, Write};

fn longest_common_subsequence(s1: &str, s2: &str) -> String {
    let n = s1.len();
    let m = s2.len();
    
    // Create a 2D vector to store lengths of longest common subsequence.
    let mut lcs_table = vec![vec![0; m + 1]; n + 1];

    // Build the LCS table in a bottom-up manner.
    for i in 1..=n {
        for j in 1..=m {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                lcs_table[i][j] = lcs_table[i - 1][j - 1] + 1;
            } else {
                lcs_table[i][j] = lcs_table[i - 1][j].max(lcs_table[i][j - 1]);
            }
        }
    }

    // Reconstruct the LCS from the table.
    let mut lcs = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
            lcs.push(s1.as_bytes()[i - 1]);
            i -= 1;
            j -= 1;
        } else if lcs_table[i - 1][j] >= lcs_table[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // The LCS is built in reverse order, so reverse it.
    lcs.reverse();
    String::from_utf8(lcs).unwrap_or_default() // Convert to String, return empty if invalid UTF-8
}

fn main() {
    let mut input = String::new();
    
    // Read first string
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s1 = input.trim();
    
    // Check length precondition
    if s1.len() > 2999 {
        panic!("Input string exceeds maximum length");
    }

    input.clear(); // Clear input for the second string

    // Read second string
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s2 = input.trim();
    
    // Check length precondition
    if s2.len() > 2999 {
        panic!("Input string exceeds maximum length");
    }

    // Compute the longest common subsequence
    let lcs = longest_common_subsequence(s1, s2);
    
    // Output the result
    println!("{}", lcs);
}
