use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read the number of elements
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // If n is zero, output zero immediately
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let mut h: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        h.push(value);
    }

    // Initialize variables for tracking the longest non-increasing subsequence
    let mut max_length = 0;
    let mut current_length = 0;

    // Iterate through the sequence to find the longest non-increasing subsequence
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
