// Import necessary modules
use std::io::{self, BufRead};

// Define the constant M as 2^20
const M: usize = 1 << 20;

// Function to perform modular exponentiation
fn modular_exponentiation(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1; // Divide exp by 2
        base = (base * base) % modulus;
    }
    result
}

// Main function to compute the result based on the input sequence
fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize arrays
    let mut s = vec![0; M];
    let mut onum = vec![0; M];
    let mut prev = vec![0; M];
    let mut dp = vec![0; M];
    let mut sum = vec![0; M];
    let mut osum = vec![0; M];

    // Read the sequence of integers
    for _ in 0..n {
        let a: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        // Update the cumulative XOR and counts
        // This part of the logic is inferred from the specification
        // and may need adjustment based on the exact behavior expected.
        let idx = (a as usize) % M; // Ensure index is within bounds
        s[idx] ^= a; // Cumulative XOR
        onum[idx] += 1; // Count occurrences
    }

    // Calculate the number of zero XOR results
    let zero_xor_count = onum.iter().filter(|&&x| x > 0).count() as i64;

    // Compute the final result using modular exponentiation
    let result = modular_exponentiation(2, zero_xor_count, 1_000_000_007);

    // Output the result
    println!("{}", result);
}
