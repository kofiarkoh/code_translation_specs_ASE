fn longest_non_increasing_subsequence(n: usize, sequence: &[i32]) -> usize {
    // Preconditions: n must be non-negative and sequence must contain n integers.
    // If n is 0, the longest non-increasing subsequence is 0.
    if n == 0 {
        return 0;
    }

    // Initialize variables to track the maximum length and current length of the subsequence.
    let mut max_length = 1; // At least one element means a length of 1.
    let mut current_length = 1;

    // Iterate through the sequence to find the longest non-increasing contiguous subsequence.
    for i in 1..n {
        // Compare the current element with the previous one.
        if sequence[i] <= sequence[i - 1] {
            // If the current element is less than or equal to the previous, increment the current length.
            current_length += 1;
        } else {
            // Otherwise, reset the current length.
            current_length = 1;
        }

        // Update the maximum length if the current length is greater.
        if current_length > max_length {
            max_length = current_length;
        }
    }

    // Return the maximum length found.
    max_length
}

fn main() {
    // Read input from standard input.
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements.
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the sequence of integers.
    let sequence: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result and print it.
    let result = longest_non_increasing_subsequence(n, &sequence);
    println!("{}", result);
}
