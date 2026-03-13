use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut rank = [0; 3]; // Array to hold the top three unique values
    let mut unique_values = std::collections::HashSet::new(); // To track unique values

    // Read exactly 10 integers from input
    for _ in 0..10 {
        let mut input = String::new();
        handle.read_line(&mut input).expect("Failed to read line");
        let work: i32 = input.trim().parse().expect("Input is not a valid integer");

        // Insert the value into the set of unique values
        unique_values.insert(work);
    }

    // Convert the HashSet to a Vec and sort it in descending order
    let mut sorted_unique: Vec<i32> = unique_values.into_iter().collect();
    sorted_unique.sort_unstable_by(|a, b| b.cmp(a));

    // Fill the rank array with the top three unique values or zeros if not enough unique values
    for (i, &value) in sorted_unique.iter().take(3).enumerate() {
        rank[i] = value;
    }

    // Print the results
    for &value in &rank {
        println!("{}", value);
    }
}
