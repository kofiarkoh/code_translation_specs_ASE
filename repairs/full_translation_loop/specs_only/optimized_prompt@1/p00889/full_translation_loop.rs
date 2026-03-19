use std::collections::HashMap;
use std::io::{self, BufRead};

fn process_integers(n: usize, s: i32, w: i32, q: i32) -> i32 {
    // Preconditions: n must be greater than zero
    if n == 0 {
        return 0; // No processing if n is zero
    }

    let mut count = 0;
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    // Simulate processing of n integers based on s, w, and q
    for i in 0..n {
        // Here we derive a value based on the iteration index and the parameters
        let value = s + (i as i32 * w) + q; // Example computation based on s, w, q

        // Update the frequency map
        let entry = frequency_map.entry(value).or_insert(0);
        *entry += 1;

        // Count specific conditions (for example, counting unique values)
        if *entry == 1 {
            count += 1; // Increment count for each unique value
        }
    }

    // Postconditions: return the computed count
    count
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Ensure we have exactly 4 integers
        if nums.len() == 4 {
            let n = nums[0] as usize;
            let s = nums[1];
            let w = nums[2];
            let q = nums[3];

            // Process the integers and print the result
            let result = process_integers(n, s, w, q);
            println!("{}", result);
        }
    }
}
