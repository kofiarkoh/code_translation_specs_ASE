use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Edge case: if n is 0, output 0
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Initialize the dp array to store the longest increasing subsequence
    let mut dp: Vec<i32> = Vec::new();

    for &value in &a {
        // Perform binary search to find the insertion point
        let pos = dp.binary_search(&value).unwrap_or_else(|e| e);
        // If pos is equal to the length of dp, it means we can extend the subsequence
        if pos == dp.len() {
            dp.push(value);
        } else {
            // Otherwise, we replace the existing value to maintain the smallest possible tail
            dp[pos] = value;
        }
    }

    // The length of the longest increasing subsequence is the size of dp
    println!("{}", dp.len());
}
