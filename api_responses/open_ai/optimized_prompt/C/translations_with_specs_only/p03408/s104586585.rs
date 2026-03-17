use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read the first integer N
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().parse().unwrap();
    // Preconditions state N is non-negative, so we assume valid input as per the spec.

    // Read the first set of strings
    let mut first_set = Vec::new();
    for _ in 0..n {
        let string = input.next().unwrap().unwrap();
        // Each string must be at most 10 characters long
        assert!(string.len() <= 10);
        first_set.push(string);
    }

    // Read the second integer M
    let m: usize = input.next().unwrap().unwrap().parse().unwrap();
    // Preconditions state M is non-negative, so we assume valid input as per the spec.

    // Read the second set of strings
    let mut second_set = Vec::new();
    for _ in 0..m {
        let string = input.next().unwrap().unwrap();
        // Each string must be at most 10 characters long
        assert!(string.len() <= 10);
        second_set.push(string);
    }

    // Count frequencies of strings in the first set
    let mut frequency_map: HashMap<&String, usize> = HashMap::new();
    for string in &first_set {
        *frequency_map.entry(string).or_insert(0) += 1;
    }

    // Adjust frequencies based on occurrences in the second set
    for string in &second_set {
        if let Some(count) = frequency_map.get_mut(string) {
            *count = 0; // Adjust frequency to 0 if the string is found in the second set
        }
    }

    // Find the maximum adjusted frequency
    let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);

    // Output the maximum adjusted frequency
    println!("{}", max_frequency);
}
