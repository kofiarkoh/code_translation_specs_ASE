fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize a vector to hold the integers
    let mut numbers = Vec::with_capacity(n);

    // Read the subsequent lines for the integers
    for _ in 0..n {
        let number: i64 = lines.next().unwrap().unwrap().parse().unwrap(); // Assuming long integer is represented by i64
        // Apply transformation (if any) here; currently, we just store the number
        numbers.push(number);
    }

    // Compute the minimum value from the processed integers
    // The specification guarantees at least one integer, so we can safely unwrap
    let min_value = *numbers.iter().min().unwrap();

    // Output the computed minimum value
    // The output is guaranteed to be non-negative as per the specification
    println!("{}", min_value);
}
