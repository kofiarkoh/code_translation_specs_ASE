use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if nums.is_empty() || nums[0] == 0 {
            break;
        }

        let n = nums.remove(0); // Remove the first element which is the count
        if n == 0 {
            break; // Termination condition
        }

        // Ensure we have the correct number of integers
        if nums.len() != n as usize {
            continue; // Skip malformed input
        }

        let mut max_length = 0;
        let mut most_frequent = None;

        // Sort the numbers
        nums.sort_unstable();

        let mut i = 0;
        while i < n as usize {
            let mut j = i + 1;

            // Find the length of the contiguous subarray with the same value
            while j < n as usize && nums[j] == nums[i] {
                j += 1;
            }

            let length = j - i;
            if length > max_length {
                max_length = length;
                most_frequent = Some(nums[i]);
            }

            // Move to the next distinct number
            i = j;
        }

        // Check the condition for output
        if max_length > n as usize / 2 {
            if let Some(value) = most_frequent {
                println!("{}", value);
            }
        } else {
            println!("NO COLOR");
        }
    }
}
