// Aizu Vol0 0049: Blood Groups
// Translated from C to Rust

use std::io::{self, BufRead};
use std::collections::HashMap;

const MAX: usize = 50;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Initialize blood group counts
    let mut blood: [Option<u8>; MAX + 1] = [None; MAX + 1];
    let mut cnt: [usize; 5] = [0; 5];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();

        // Read unique identifier
        let id: usize = match parts.next() {
            Some(num_str) => num_str.parse().expect("Invalid identifier"),
            None => continue,
        };

        // Check if the identifier is within the valid range
        if id > MAX {
            continue; // Ignore out-of-range identifiers
        }

        // Read blood group
        let blood_group = match parts.next() {
            Some(group) => group,
            None => continue,
        };

        let t = match blood_group {
            "AB" => 3,
            "B" => 2,
            "O" => 4,
            "A" => 1,
            _ => continue, // Ignore invalid blood groups
        };

        // Check for conflicts
        if blood[id].is_none() {
            blood[id] = Some(t);
            cnt[t] += 1;
        } else if blood[id] != Some(t) {
            loop {
                // Infinite loop on conflict
            }
        }
    }

    // Output the counts for each blood group
    for i in 1..5 {
        println!("{}", cnt[i]);
    }
}
