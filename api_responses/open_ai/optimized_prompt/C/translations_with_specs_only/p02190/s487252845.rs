use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Collect all input integers
    let mut input_numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok()) // Parse integers, ignore invalid inputs
            .collect();
        input_numbers.extend(numbers);
    }

    // The first integer indicates the number of subsequent integers
    let count = input_numbers.get(0).cloned().unwrap_or(0) as usize;

    // Ensure we only process the specified number of integers
    let distinct_count = if count > 0 {
        let mut distinct_numbers: HashSet<i32> = HashSet::new();
        
        // Insert subsequent integers into the HashSet to ensure uniqueness
        for &num in &input_numbers[1..=count.min(input_numbers.len() - 1)] {
            distinct_numbers.insert(num);
        }
        
        // The number of distinct integers is the size of the HashSet
        distinct_numbers.len()
    } else {
        0 // If count is 0, there are no subsequent integers to process
    };

    // Output the count of distinct integers
    println!("{}", distinct_count);
}
