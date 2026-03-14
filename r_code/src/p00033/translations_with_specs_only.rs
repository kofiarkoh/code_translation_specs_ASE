fn main() {
    use std::io::{self, BufRead};

    // Read all input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of sequences
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each sequence
    for _ in 0..n {
        // Read the next line of ten integers
        let sequence: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Use a HashSet to track distinct values
        let mut distinct_values = std::collections::HashSet::new();

        // Collect distinct values from the sequence
        for &value in &sequence {
            distinct_values.insert(value);
            // Early exit if we already have more than 2 distinct values
            if distinct_values.len() > 2 {
                break;
            }
        }

        // Output "YES" if there are at most 2 distinct values, otherwise "NO"
        if distinct_values.len() <= 2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
