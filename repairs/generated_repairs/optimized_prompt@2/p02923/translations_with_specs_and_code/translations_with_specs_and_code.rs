use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Keep as mutable

    // Read the number of elements
    let mut n = String::new();
    handle.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Input must be a non-negative integer");

    // Initialize the vector to hold the sequence
    let mut h = vec![0; n];

    // Read the sequence of integers
    for i in 0..n {
        let mut input = String::new();
        handle.read_line(&mut input).expect("Failed to read line");
        h[i] = input.trim().parse().expect("Input must be an integer");
    }

    // Variables to track the maximum length and current count
    let mut max_length = 0;
    let mut count = 1; // Start count at 1 to include the first element

    // Iterate through the sequence to find the longest non-increasing subsequence
    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            count += 1;
        } else {
            max_length = max_length.max(count);
            count = 1; // Reset count to 1 for the current element
        }
    }
    max_length = max_length.max(count); // Check at the end for the last subsequence

    // Output the result
    println!("{}", max_length); // No need to add 1, as count already includes the first element
}
