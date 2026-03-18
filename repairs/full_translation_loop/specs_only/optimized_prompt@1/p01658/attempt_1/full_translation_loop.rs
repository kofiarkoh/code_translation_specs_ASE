use std::io::{self, BufRead};

fn main() {
    // Read dimensions M and N from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read M and N
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let m = dimensions[0];
    let n = dimensions[1];

    // Initialize the transition count
    let mut transition_count = 0;

    // Process each row of the matrix
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<u8> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Count transitions from 1 to 0 in the current row
        for j in 1..n {
            if row[j - 1] == 1 && row[j] == 0 {
                transition_count += 1;
            }
        }
    }

    // Output the total count of transitions
    println!("{}", transition_count);
}
