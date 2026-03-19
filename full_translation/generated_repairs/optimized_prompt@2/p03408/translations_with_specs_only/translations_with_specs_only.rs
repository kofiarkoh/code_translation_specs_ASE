use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read the number of strings in the first set
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for number of strings in the first set");

    // Read the first set of strings
    let mut first_set = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().lock().read_line(&mut input).unwrap();
        let s = input.trim().to_string();
        assert!(s.len() <= 10, "String exceeds maximum length of 10 characters");
        first_set.push(s);
    }

    // Read the number of strings in the second set
    input.clear();
    io::stdin().lock().read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().expect("Invalid input for number of strings in the second set");

    // Read the second set of strings
    let mut second_set = HashMap::new();
    for _ in 0..m {
        input.clear();
        io::stdin().lock().read_line(&mut input).unwrap();
        let s = input.trim().to_string();
        assert!(s.len() <= 10, "String exceeds maximum length of 10 characters");
        *second_set.entry(s).or_insert(0) += 1; // Count occurrences of each string in the second set
    }

    // Count frequencies of strings in the first set
    let mut frequency = HashMap::new();
    for s in first_set {
        *frequency.entry(s).or_insert(0) += 1; // Count occurrences of each string in the first set
    }

    // Calculate the maximum adjusted frequency
    let max_adjusted_frequency = frequency.iter()
        .map(|(s, &count)| {
            // Adjust frequency by the presence of matching strings in the second set
            let adjustment: i32 = *second_set.get(s).unwrap_or(&0); // Specify type for adjustment
            count.saturating_sub(adjustment as i32) // Ensure non-negative result
        })
        .max()
        .unwrap_or(0); // If no strings, max frequency is 0

    // Output the result
    println!("{}", max_adjusted_frequency);
}
