use std::io::{self, BufRead};

fn solve(n: usize, a: Vec<i64>, b: Vec<i64>) -> i32 {
    // Edge case: if n is 0, we should return "Yes 0"
    if n == 0 {
        println!("Yes 0");
        return 0;
    }

    // Dynamic programming table
    let mut dp = vec![0; n];

    // Initialize dp based on the input pairs
    for i in 0..n {
        dp[i] = a[i] + b[i]; // Compute the sum of each pair
    }

    // Calculate the total sum of dp
    let total_sum: i64 = dp.iter().sum();

    // The expected output is just the total sum
    println!("{}", total_sum);
    
    // Return 0 for success
    0
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the pairs into vectors
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        a.push(nums[0]);
        b.push(nums[1]);
    }

    // Call the solve function
    let _ = solve(n, a, b);
}
