use std::io::{self, BufRead};

fn compute_value(n: u64, k: u64) -> u64 {
    // Base case: if n is 0, return 1 (as a simple example of a recursive function)
    if n == 0 {
        return 1;
    }
    // Recursive case: compute the value based on n and k
    // Here we assume a simple recursive relationship for demonstration
    // This could be adjusted based on the actual specification of the computation
    compute_value(n - 1, k) + k
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs = handle.lines();

    // Read the first input (n)
    let n: u64 = inputs.next().unwrap().unwrap().parse().unwrap();
    // Read the second input (k)
    let k: u64 = inputs.next().unwrap().unwrap().parse().unwrap();

    // Ensure inputs are non-negative as per the specification
    assert!(n >= 0);
    assert!(k >= 0);

    // Compute the value and print it
    let result = compute_value(n, k);
    println!("{}", result);
}
