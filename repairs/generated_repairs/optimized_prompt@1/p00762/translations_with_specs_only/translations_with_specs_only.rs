// This program simulates the placement of blocks on a grid based on specified orientations
// and counts the occurrences of each block orientation after all placements.

use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Initialize a vector to count occurrences of orientations from 1 to 6
    let mut orientation_counts = vec![0; 6];

    // Read the first line to get the number of blocks
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let num_blocks: usize = first_line.trim().parse().expect("Invalid number of blocks");

    // Process each block's orientations
    for _ in 0..num_blocks {
        let line = lines.next().unwrap().unwrap();
        let orientations: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid orientation"))
            .collect();

        // Each block has a top and front orientation
        if orientations.len() == 2 {
            let top = orientations[0];
            let front = orientations[1];

            // Ensure orientations are within the valid range of 1 to 6
            if (1..=6).contains(&top) && (1..=6).contains(&front) {
                orientation_counts[top - 1] += 1; // Increment count for top orientation
                orientation_counts[front - 1] += 1; // Increment count for front orientation
            }
        }
    }

    // Read the number of queries
    let query_line = lines.next().unwrap().unwrap();
    let num_queries: usize = query_line.trim().parse().expect("Invalid number of queries");

    // Process each query
    for _ in 0..num_queries {
        let query_line = lines.next().unwrap().unwrap();
        let query: Vec<usize> = query_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid query"))
            .collect();

        // Output the counts for the queried orientations
        let output: Vec<String> = query.iter()
            .map(|&q| orientation_counts[q - 1].to_string())
            .collect();
        
        println!("{}", output.join(" "));
    }
}
