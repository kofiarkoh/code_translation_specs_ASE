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

    // Initialize a variable to hold the previous row's last value
    let mut previous_row_last_value: Option<u8> = None;

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

        // Count transition from the last value of the previous row to the first value of the current row
        if let Some(prev_last) = previous_row_last_value {
            if prev_last == 1 && row[0] == 0 {
                transition_count += 1;
            }
        }

        // Update the last value of the current row for the next iteration
        previous_row_last_value = Some(*row.last().unwrap());
    }

    // Output the total count of transitions
    println!("{}", transition_count);
}
