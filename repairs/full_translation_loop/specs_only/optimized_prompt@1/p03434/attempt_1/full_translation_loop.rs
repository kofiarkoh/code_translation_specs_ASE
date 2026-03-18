use std::io::{self, BufRead};

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Read the sequence of integers
    let mut sequence: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().expect("Invalid integer"))
        .collect();

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
