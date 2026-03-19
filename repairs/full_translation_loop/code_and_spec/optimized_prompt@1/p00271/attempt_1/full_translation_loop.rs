use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines();

    // We expect exactly 7 pairs of integers, so we will read 14 integers in total.
    for _ in 0..7 {
        // Read a line and expect it to contain two integers
        let line = input.next().expect("Expected more input").expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse integer"))
            .collect();

        // Ensure we have exactly 2 integers
        assert_eq!(nums.len(), 2, "Expected exactly two integers in the input");

        let a = nums[0];
        let b = nums[1];

        // Output the difference
        println!("{}", a - b);
    }
}
