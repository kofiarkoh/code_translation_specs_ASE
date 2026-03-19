use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read the number of elements
    let mut n = String::new();
    handle.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Input must be a non-negative integer");

    // Initialize the vector to hold the sequence
    let mut h = Vec::with_capacity(n);

    // Read the sequence of integers
    for _ in 0..n {
        let mut input = String::new();
        handle.read_line(&mut input).expect("Failed to read line");
        let value: i32 = input.trim().parse().expect("Input must be an integer");
        h.push(value);
    }

    // Variables to track the maximum length and current count
    let mut max_length = 0;
    let mut count = 0;

    // Iterate through the sequence to find the longest non-increasing subsequence
    for i in 0..n {
        if i > 0 && h[i] <= h[i - 1] {
            count += 1;
        } else {
            count = 0;
        }
        max_length = max_length.max(count);
    }

    // Output the result
    println!("{}", max_length + 1); // +1 to account for the first element in the subsequence
}
