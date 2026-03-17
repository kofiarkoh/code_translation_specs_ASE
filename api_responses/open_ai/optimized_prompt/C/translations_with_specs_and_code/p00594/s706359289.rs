use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent integers
        let n = match nums.get(0) {
            Some(&count) if count > 0 => count as usize,
            _ => continue, // Skip if the first number is not valid
        };

        // If the count is not matching the length of the input, skip
        if nums.len() != n + 1 {
            continue;
        }

        // Process the integers
        let mut frequency = HashMap::new();
        for &num in &nums[1..=n] {
            *frequency.entry(num).or_insert(0) += 1;
        }

        // Find the most frequent integer
        let mut most_frequent = None;
        let mut max_count = 0;

        for (&num, &count) in &frequency {
            if count > max_count {
                max_count = count;
                most_frequent = Some(num);
            }
        }

        // Check if the most frequent integer's count exceeds half of n
        if max_count > n / 2 {
            if let Some(num) = most_frequent {
                println!("{}", num);
            }
        } else {
            println!("NO COLOR");
        }
    }
}
