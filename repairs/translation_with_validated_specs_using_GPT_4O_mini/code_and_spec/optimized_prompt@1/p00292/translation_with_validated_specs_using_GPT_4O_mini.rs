use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Expected a non-negative integer"),
        _ => panic!("Failed to read the number of pairs"),
    };

    // Process each pair
    for _ in 0..n {
        let line = lines.next().expect("Expected a pair of integers");
        let pair: Vec<i32> = line
            .expect("Failed to read line")
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Expected an integer"))
            .collect();

        if pair.len() != 2 {
            panic!("Expected exactly two integers for each pair");
        }

        let x = pair[0];
        let y = pair[1];

        // Compute the derived value
        let result = (x - 1) % y + 1;

        // Output the result
        println!("{}", result);
    }
}
