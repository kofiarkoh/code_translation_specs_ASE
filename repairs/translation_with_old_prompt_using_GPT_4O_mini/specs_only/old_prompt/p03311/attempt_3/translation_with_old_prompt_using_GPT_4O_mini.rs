use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure n is greater than 0
    assert!(n > 0, "n must be greater than 0");

    // Read the integers into a vector
    let mut a: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Adjust each integer by subtracting its index
    for i in 0..n {
        a[i] -= i as i64; // Adjusting the integer
    }

    // Sort the adjusted integers
    a.sort();

    // Calculate the minimum sadness based on the differences
    let mut min_sadness = i64::MAX; // Initialize to maximum possible value
    for i in 1..n {
        let sadness = a[i] - a[i - 1]; // Calculate the difference
        if sadness < min_sadness {
            min_sadness = sadness; // Update minimum sadness
        }
    }

    // If n is 1, we should not output 0; instead, we should handle it as a special case
    if n == 1 {
        min_sadness = 0; // This case is not meaningful for sadness calculation
    }

    // Output the result
    println!("{}", min_sadness);
}
