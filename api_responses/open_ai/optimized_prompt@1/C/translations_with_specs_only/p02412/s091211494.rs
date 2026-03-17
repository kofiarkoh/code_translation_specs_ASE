use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 2 {
            continue; // Skip invalid input lines
        }

        let n = nums[0];
        let x = nums[1];

        // Termination condition
        if n == 0 && x == 0 {
            break;
        }

        // Count unique combinations of three distinct integers
        let count = count_unique_combinations(n, x);
        println!("{}", count);
    }
}

// Function to count unique combinations of three distinct integers from 1 to n that sum to x
fn count_unique_combinations(n: i32, x: i32) -> i32 {
    let mut count = 0;

    // Iterate through all combinations of three distinct integers
    for i in 1..=n {
        for j in (i + 1)..=n {
            for k in (j + 1)..=n {
                if i + j + k == x {
                    count += 1;
                }
            }
        }
    }

    count // Return the count of unique combinations
}
