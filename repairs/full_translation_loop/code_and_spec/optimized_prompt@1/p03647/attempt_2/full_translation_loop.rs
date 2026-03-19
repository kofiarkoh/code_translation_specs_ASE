use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first input which indicates the number of iterations
    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.trim().parse().expect("Invalid input for number of iterations");

    // Initialize a pointer-like structure to track the state
    let mut p = 0;
    let mut possible = true;

    // Process pairs of integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for pair of integers"))
            .collect();

        if nums.len() != 2 {
            // If the input does not conform to the expected format, we terminate
            possible = false;
            break;
        }

        let (a, b) = (nums[0], nums[1]);

        // Evaluate the conditions based on the input pairs
        if a != 0 {
            if b != n {
                p = 0;
            } else {
                p += 1;
            }
        } else {
            p = b;
        }
    }

    // Output the result based on the evaluation
    if p > 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
