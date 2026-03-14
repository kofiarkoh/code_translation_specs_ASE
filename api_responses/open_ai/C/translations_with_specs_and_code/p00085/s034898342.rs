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
            continue; // Skip invalid lines, though spec states input is valid
        }

        let (n, m) = (nums[0], nums[1]);
        if n == 0 && m == 0 {
            break; // Termination condition
        }

        let mut d = vec![1; n as usize]; // Initialize the array with 1s
        let mut i = 0;
        let mut sum = 0;
        let mut f = 0;

        while f < n {
            sum += d[i];
            if sum == m {
                d[i] = 0; // Mark this position as contributing to the target sum
                sum = 0; // Reset sum after reaching target
                f += 1; // Increment the count of contributions
            }
            i = (i + 1) % n; // Move to the next index, wrapping around
        }

        // Output the position (1-based index)
        println!("{}", i + 1);
    }
}
