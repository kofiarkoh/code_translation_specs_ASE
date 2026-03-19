use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for the total number of elements
    let mut lines = reader.lines();
    let total_elements: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the second line for the number of pairs to process
    let num_pairs: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Prepare a vector to store the computed values
    let mut results = Vec::new();

    // Process each pair of integers
    for _ in 0..num_pairs {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Ensure we have exactly two integers in the pair
        if nums.len() != 2 {
            continue; // Behavior is not defined for invalid pairs
        }

        let a = nums[0];
        let b = nums[1];

        // Check if the integers are within the specified range
        if a < 1 || a > total_elements as i32 || b < 1 || b > total_elements as i32 {
            continue; // Behavior is not defined for out-of-range values
        }

        // Compute the value based on the relationship of a and b
        let computed_value = if a == b {
            1
        } else if (a + b) % 2 == 0 {
            2
        } else {
            3
        };

        // Store the computed value
        results.push(computed_value);
    }

    // Output the computed values
    for result in results {
        println!("{}", result);
    }
}
