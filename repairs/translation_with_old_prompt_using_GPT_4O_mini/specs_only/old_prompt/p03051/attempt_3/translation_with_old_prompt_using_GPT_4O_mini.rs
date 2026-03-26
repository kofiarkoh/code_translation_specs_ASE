use std::io::{self, BufRead};

const M: usize = 1 << 20; // M = 2^20
const MODULUS: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0),
        _ => return, // Exit if input is invalid
    };

    // Initialize arrays
    let mut s = vec![0; M];
    let mut onum = vec![0; M];
    let mut prev = vec![0; M];
    let mut dp = vec![0; M];
    let mut sum = vec![0; M];
    let mut osum = vec![0; M];

    // Read the sequence of integers
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(Ok(value)) = lines.next() {
            if let Ok(num) = value.parse::<i64>() {
                a.push(num);
            }
        }
    }

    // Compute cumulative XOR values and count occurrences of zero XOR results
    let mut xor_sum = 0;
    let mut zero_count = 0;

    // We need to count the number of times the cumulative XOR is zero
    for &value in &a {
        xor_sum ^= value;
        if xor_sum == 0 {
            zero_count += 1;
        }
    }

    // Additionally, we need to account for the initial state (before any elements)
    // If the cumulative XOR is zero at the start, we count that as well
    if xor_sum == 0 {
        zero_count += 1;
    }

    // The final result is based on the zero_count
    let result = modular_exponentiation(2, zero_count as i64 - 1, MODULUS);
    
    // Output the result
    println!("{}", result);
}

// Function to perform modular exponentiation
fn modular_exponentiation(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus;
        e /= 2;
    }
    result
}
