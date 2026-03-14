// Aizu Vol-0 0097: Sum of Integers II
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX_SUM: usize = 864;
const MAX_N: usize = 9; // The maximum number of summands
const MAX_I: usize = 101; // The maximum value for summands
const MAX_J: usize = 902; // The maximum target sum

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Initialize the dynamic programming table
    let mut p = [[[0; MAX_J]; MAX_I]; MAX_N];

    // Base case: one way to sum to each value with one summand
    for i in 0..=100 {
        p[1][i][i] = 1;
    }

    // Fill the DP table according to the logic in the C code
    for n in 1..8 { // n goes from 1 to 8 (inclusive)
        let min = n - 1;
        let max = n * 100;
        for i in 0..=100 {
            for j in min..=max {
                for k in 1..=100 - i {
                    p[n + 1][i + k][j + i + k] += p[n][i][j];
                }
            }
        }
    }

    // Read input and process each case
    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let s = nums.next().unwrap();

        // Check preconditions
        if n == 0 {
            break; // Terminate if the number of summands is zero or negative
        }

        // Output according to the specification
        if s > MAX_SUM {
            println!("0");
        } else {
            let ans: usize = (0..=100).map(|i| p[n][i][s]).sum();
            println!("{}", ans);
        }
    }
}
