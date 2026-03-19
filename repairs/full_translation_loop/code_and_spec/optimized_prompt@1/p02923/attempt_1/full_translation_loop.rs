use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of elements
    let mut n_str = String::new();
    handle.read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Input must be a non-negative integer");

    // Preallocate a vector for the sequence
    let mut h = vec![0; n];

    // Read the sequence of integers
    for i in 0..n {
        let mut num_str = String::new();
        handle.read_line(&mut num_str).expect("Failed to read line");
        h[i] = num_str.trim().parse().expect("Input must be an integer");
    }

    let mut max_length = 0;
    let mut current_length = 0;

    // Compute the length of the longest non-increasing contiguous subsequence
    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            current_length += 1;
        } else {
            current_length = 0;
        }
        max_length = max_length.max(current_length);
    }

    // Output the result
    println!("{}", max_length);
}
