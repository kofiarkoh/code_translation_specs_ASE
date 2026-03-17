fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line which indicates the number of pairs
    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.parse().expect("First input must be a valid integer");

    // Initialize a flag to determine if the condition is possible
    let mut possible = false;

    // Iterate over the number of pairs
    for _ in 0..n {
        // Read the next line containing a pair of integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Subsequent inputs must be pairs of valid integers"))
            .collect();

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            // If not, we terminate unexpectedly as per the specification
            panic!("Expected a pair of integers");
        }

        // Evaluate the condition based on the pair of integers
        // Here we assume a simple condition for demonstration purposes
        // This can be modified based on the actual condition to check
        if nums[0] + nums[1] > 0 {
            possible = true;
        }
    }

    // Output the result based on the evaluation
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
