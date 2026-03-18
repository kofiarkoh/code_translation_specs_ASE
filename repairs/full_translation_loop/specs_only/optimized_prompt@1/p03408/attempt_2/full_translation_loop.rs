use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of strings in the first set
    let mut first_set_count = String::new();
    reader.read_line(&mut first_set_count).unwrap();
    let first_set_count: usize = first_set_count.trim().parse().unwrap();

    // Read the strings in the first set
    let mut first_set = Vec::new();
    for _ in 0..first_set_count {
        let mut string = String::new();
        reader.read_line(&mut string).unwrap();
        let string = string.trim().to_string();
        assert!(string.len() <= 10); // Enforce maximum length constraint
        first_set.push(string);
    }

    // Read the number of strings in the second set
    let mut second_set_count = String::new();
    reader.read_line(&mut second_set_count).unwrap();
    let second_set_count: usize = second_set_count.trim().parse().unwrap();

    // Read the strings in the second set
    let mut second_set = HashMap::new();
    for _ in 0..second_set_count {
        let mut string = String::new();
        reader.read_line(&mut string).unwrap();
        let string = string.trim().to_string();
        assert!(string.len() <= 10); // Enforce maximum length constraint
        *second_set.entry(string).or_insert(0) += 1; // Count occurrences
    }

    // Count frequencies of strings in the first set
    let mut frequency = HashMap::new();
    for string in first_set {
        *frequency.entry(string).or_insert(0) += 1; // Count occurrences
    }

    // Calculate the maximum adjusted frequency
    let max_adjusted_frequency = frequency.iter()
        .map(|(string, count)| {
            let adjusted_count = count - second_set.get(string).unwrap_or(&0); // Use reference to string
            adjusted_count.max(0) // Ensure non-negative frequency
        })
        .max()
        .unwrap_or(0); // Default to 0 if no strings

    // Output the result
    println!("{}", max_adjusted_frequency);
}
