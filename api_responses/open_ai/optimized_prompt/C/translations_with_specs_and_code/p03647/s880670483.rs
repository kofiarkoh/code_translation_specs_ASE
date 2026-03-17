use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for the number of iterations
    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.trim().parse().expect("Invalid integer input");

    let mut possible = true;
    let mut p = 0; // This will act as a pointer/index

    // Process pairs of integers
    for line in lines {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid integer input"))
            .collect();

        if nums.len() != 2 {
            // If the input format is incorrect, we do not handle it explicitly as per the spec
            break;
        }

        let (a, b) = (nums[0], nums[1]);

        // Evaluate the conditions based on the input pairs
        if p != 0 && n != 0 {
            possible = false;
            break;
        }

        if a != 0 {
            if b != n {
                p = 0; // Reset p if conditions are not met
            } else {
                p = 1; // Set p to 1 if conditions are met
            }
        } else {
            p = b; // Set p to b if a is 0
        }
    }

    // Output the result based on the evaluation
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
