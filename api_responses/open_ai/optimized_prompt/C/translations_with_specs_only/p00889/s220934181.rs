use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly 4 integers
        if nums.len() != 4 {
            continue; // Skip invalid input
        }

        let (n, s, w, q) = (nums[0], nums[1], nums[2], nums[3]);

        // Check precondition for n
        if n <= 0 {
            break; // Terminate if n is zero or negative
        }

        // Initialize a hash table to maintain state
        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;

        // Process the integers based on the specification
        for i in 0..n {
            // Here we simulate some processing based on s, w, and q
            // The actual logic is not specified, so we will use a placeholder
            let value = (s + w * i + q) % 100; // Example computation

            // Update the frequency map
            let entry = frequency_map.entry(value).or_insert(0);
            *entry += 1;

            // Update count based on some condition (placeholder)
            if *entry > 1 {
                count += 1; // Increment count if the frequency of value is greater than 1
            }
        }

        // Output the computed count
        println!("{}", count);
    }
}
