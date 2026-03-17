fn main() {
    use std::io::{self, BufRead};

    // Read the number of pairs from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize a variable to hold the computed result
    let mut computed_result: i64 = 0;

    // Process each pair of integers
    for _ in 0..n {
        // Read the line containing the pair of integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Assuming the input is always valid as per the specification
        // Each line should contain exactly two integers
        let a = nums[0];
        let b = nums[1];

        // Perform the intended calculation
        computed_result += a + b; // Change to addition as per expected behavior
    }

    // Print the computed result
    println!("{}", computed_result);
}
