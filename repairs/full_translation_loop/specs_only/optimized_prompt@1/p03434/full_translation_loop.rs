use std::io::{self, BufRead};

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Read the sequence of integers
    let mut sequence: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        // Read each line and parse it as an integer
        if let Some(Ok(line)) = lines.next() {
            let trimmed_line = line.trim(); // Create a longer-lived value
            // Handle potential parsing errors gracefully
            match trimmed_line.parse::<i32>() {
                Ok(value) => sequence.push(value),
                Err(_) => {
                    eprintln!("Invalid integer: {}", trimmed_line);
                    return; // Exit if there's an invalid integer
                }
            }
        }
    }

    // Sort the sequence in non-decreasing order
    sequence.sort();

    // Initialize sums for even and odd indices
    let mut sum_even = 0;
    let mut sum_odd = 0;

    // Compute sums based on sorted sequence
    for (i, &value) in sequence.iter().enumerate() {
        if i % 2 == 0 {
            sum_even += value; // Sum for even indices
        } else {
            sum_odd += value; // Sum for odd indices
        }
    }

    // Compute the difference
    let difference = sum_even - sum_odd;

    // Output the result
    println!("{}", difference);
}
