use std::io::{self, BufRead};

fn compute_value(n: u64, k: u64) -> u64 {
    // Base case: if n is 0, return 1 (as a simple example of a recursive function)
    if n == 0 {
        return 1;
    }
    // Recursive case: compute the value based on n and k
    // This assumes a simple recursive relationship for demonstration
    compute_value(n - 1, k) + k
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs = handle.lines();

    // Read the first input (n)
    let n: u64 = match inputs.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 on parse error
        _ => return, // Exit if no input is provided
    };
    // Read the second input (k)
    let k: u64 = match inputs.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 on parse error
        _ => return, // Exit if no input is provided
    };

    // Compute the value and print it
    let result = compute_value(n, k);
    println!("{}", result);
}
