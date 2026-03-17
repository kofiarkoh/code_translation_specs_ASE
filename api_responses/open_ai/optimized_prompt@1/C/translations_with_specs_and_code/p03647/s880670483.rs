use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for the number of iterations
    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.trim().parse().expect("Invalid input for number of iterations");

    let mut p = 0; // This will act as a pointer or index
    let mut possible = true; // To track if the condition is possible

    // Process pairs of integers
    for line in lines {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for pair of integers"))
            .collect();

        if nums.len() != 2 {
            // If the input does not conform to the expected format, we can terminate
            break;
        }

        let a = nums[0];
        let b = nums[1];

        // Evaluate the conditions based on the input pairs
        if p != 0 && n != 0 {
            possible = false;
            break;
        }

        if a != 0 {
            if b != n {
                p = 0; // Reset p if b is not equal to n
            } else {
                p += 1; // Increment p if b equals n
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
