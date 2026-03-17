// AOJ 2028: Gather on the Clock
// Translated from C to Rust

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read the size of the array
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut a = vec![0; n];
        
        // Read the array elements
        let input: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        a.copy_from_slice(&input);

        // Initialize the dynamic programming table
        let mut dp = vec![vec![0; n]; n];

        // Fill the dynamic programming table
        for k in 1..n {
            for i in 0..n {
                for j in 0..k {
                    let x = (i + j + 1) % n; // Circular array index
                    let mut v = (a[x] - a[i]).abs(); // Absolute difference
                    v += dp[x][k - j - 1] + dp[i][j];
                    if v > dp[i][k] {
                        dp[i][k] = v;
                    }
                }
            }
        }

        // Find the maximum value from the last column of dp
        let ans = (0..n).map(|i| dp[i][n - 1]).max().unwrap();
        println!("{}", ans);
    }
}
