use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line to get the number of pairs
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.parse().expect("Invalid input for number of pairs");

    // Initialize a flag to determine if the condition is possible
    let mut possible = true;

    // Iterate over the number of pairs
    for _ in 0..n {
        // Read each pair of integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for pair of integers"))
            .collect();

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            possible = false; // If not, we set possible to false
            break;
        }

        // Here we would evaluate the condition based on the integers
        // For the sake of this specification, we will assume a simple check
        // that both integers must be non-negative for the condition to be "POSSIBLE"
        if nums[0] < 0 || nums[1] < 0 {
            possible = false;
            break;
        }
    }

    // Output the result based on the evaluation
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
