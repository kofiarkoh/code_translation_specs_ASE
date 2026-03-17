use std::io::{self, BufRead};

const SMAX: usize = 4000; // Maximum length of input strings

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read the two binary strings from standard input
    let a = input.next().unwrap().unwrap();
    let b = input.next().unwrap().unwrap();

    // Ensure the lengths do not exceed SMAX
    assert!(a.len() <= SMAX && b.len() <= SMAX);

    // Call the function to find the lexicographically smallest interleaved string
    let result = smallest_interleaved_string(&a, &b);
    
    // Output the result
    println!("{}", result);
}

// Function to compute the lexicographically smallest interleaved binary string
fn smallest_interleaved_string(a: &str, b: &str) -> String {
    let n = a.len();
    let m = b.len();
    
    // Dynamic programming table to store results
    let mut dp = vec![vec![""; m + 1]; n + 1];

    // Fill the dp table
    for i in 0..=n {
        for j in 0..=m {
            if i == 0 && j == 0 {
                dp[i][j] = ""; // Base case: both strings are empty
            } else if i == 0 {
                dp[i][j] = &b[0..j]; // Only b is available
            } else if j == 0 {
                dp[i][j] = &a[0..i]; // Only a is available
            } else {
                // Choose the lexicographically smaller option
                let option1 = format!("{}{}", &a[i - 1..i], dp[i - 1][j]);
                let option2 = format!("{}{}", &b[j - 1..j], dp[i][j - 1]);
                dp[i][j] = if option1 < option2 { option1 } else { option2 };
            }
        }
    }

    // The result is in dp[n][m]
    dp[n][m].to_string()
}
