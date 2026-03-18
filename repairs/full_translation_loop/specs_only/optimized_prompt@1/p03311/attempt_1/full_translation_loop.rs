use std::io::{self, BufRead};

fn main() {
    // Read the number of elements from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize a vector to hold the integers
    let mut values = Vec::with_capacity(n);

    // Read the subsequent lines to get the integers
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        // Apply transformation: here we assume the transformation is identity
        // as the specification does not define any specific transformation.
        values.push(value);
    }

    // Compute the minimum value from the processed integers
    // The output must be a non-negative integer, so we filter out negatives
    let min_value = values.iter().cloned().filter(|&x| x >= 0).min().unwrap_or(0);

    // Output the computed minimum value
    println!("{}", min_value);
}
