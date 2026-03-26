use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Handle the case where n is 0
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let a: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Initialize the dp array to store the longest increasing subsequence
    let mut dp = vec![0; n];
    let mut length = 0;

    for &value in &a {
        // Perform binary search to find the insertion point
        let mut left = 0;
        let mut right = length;

        while left < right {
            let mid = (left + right) / 2;
            if dp[mid] < value {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // Update the dp array
        dp[left] = value;
        if left == length {
            length += 1; // Increase the length of the longest increasing subsequence
        }
    }

    // Output the length of the longest increasing subsequence
    println!("{}", length);
}
