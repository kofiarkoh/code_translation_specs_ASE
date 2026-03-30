use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // We expect exactly 7 pairs of integers
    for _ in 0..7 {
        if let Some(Ok(line)) = lines.next() {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // Ensure we have exactly 2 integers
            if nums.len() == 2 {
                let (a, b) = (nums[0], nums[1]);
                println!("{}", a - b);
            }
        }
    }
}
``` 

// The code above is a direct translation of the provided C code into Rust, maintaining the specified behavior and output format. It reads exactly seven pairs of integers and prints the difference for each pair. The handling of input assumes valid data as per the specification.
