use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct DataEntry {
    name: String,
    score: i32,
}

impl DataEntry {
    // Calculate the score based on the three integer values
    fn new(name: String, value1: i32, value2: i32, value3: i32) -> Self {
        // Assuming a simple weighted score calculation
        let score = value1 + value2 * 2 + value3 * 3; // Example weights
        DataEntry { name, score }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of entries
    let mut lines = reader.lines();
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // If n is zero, terminate without processing
    if n == 0 {
        return;
    }

    let mut entries: Vec<DataEntry> = Vec::new();

    // Read each entry
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 4 {
                // If the input format is incorrect, we do not define behavior
                continue;
            }

            let name = parts[0].to_string();
            let value1: i32 = parts[1].parse().unwrap_or(0);
            let value2: i32 = parts[2].parse().unwrap_or(0);
            let value3: i32 = parts[3].parse().unwrap_or(0);

            // Create a new DataEntry and push it to the vector
            let entry = DataEntry::new(name, value1, value2, value3);
            entries.push(entry);
        }
    }

    // Sort entries by score in descending order
    entries.sort_by(|a, b| b.score.cmp(&a.score));

    // Output the sorted names and scores
    for entry in entries {
        println!("{}, {}", entry.name, entry.score);
    }
}
