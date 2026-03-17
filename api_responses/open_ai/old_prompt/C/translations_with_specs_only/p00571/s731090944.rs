// Code for AOJ 0648 Art Exhibition

use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of artworks
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure N is greater than 0 as per specification
    if n == 0 {
        // Undefined behavior as per specification; we will just return
        return;
    }

    // Create a vector to hold the artworks
    let mut artworks = Vec::with_capacity(n);

    // Read the artworks' values and weights
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i64> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        // Each artwork has a value (long long) and a weight (int)
        artworks.push((parts[0], parts[1] as i32)); // Store as (value, weight)
    }

    // Sort artworks by their values
    artworks.sort_by_key(|art| art.0);

    // Initialize cumulative sums
    let mut cumulative_weight = 0;
    let mut cumulative_value = 0;
    let mut max_difference = i64::MIN; // Start with the smallest possible value

    // Calculate the maximum difference
    for (value, weight) in artworks {
        cumulative_weight += weight;
        cumulative_value += value;
        let difference = cumulative_weight - cumulative_value;
        if difference > max_difference {
            max_difference = difference;
        }
    }

    // Print the maximum difference
    println!("{}", max_difference);
}
