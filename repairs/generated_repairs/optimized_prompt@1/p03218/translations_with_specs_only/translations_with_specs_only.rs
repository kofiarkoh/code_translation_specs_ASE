use std::io::{self, BufRead};

// Function to compute the greatest common divisor (GCD) using the Euclidean algorithm
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs() // Ensure GCD is non-negative
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Preconditions: n must be positive
    assert!(n > 0, "The number of elements must be positive.");

    // Read the sequence of integers
    let mut numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Compute the product of GCDs
    let mut product: i32 = 1; // Start with multiplicative identity
    for (index, &number) in numbers.iter().enumerate() {
        let index_as_i32 = index as i32; // Convert index to i32
        let current_gcd = gcd(number, index_as_i32);
        product = (product * current_gcd) % 1_000_000_007; // Maintain product modulo the constant
    }

    // Output the result
    println!("{}", product);
}
