use std::io::{self, BufRead};

const PREDEFINED_CONSTANT: i32 = 1_000_000_007; // Example constant, can be adjusted

// Function to compute the greatest common divisor (GCD) using the Euclidean algorithm
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs() // Ensure the GCD is non-negative
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid number of elements");
    assert!(n > 0); // Precondition: n must be positive

    // Read the sequence of integers
    let mut sequence: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Invalid integer in sequence");
        sequence.push(value);
    }

    // Sort the sequence
    sequence.sort();

    // Compute the product of GCDs
    let mut product: i32 = 1;
    for (index, &value) in sequence.iter().enumerate() {
        let index_as_i32 = index as i32 + 1; // Convert index to 1-based
        product = (product * gcd(value, index_as_i32)) % PREDEFINED_CONSTANT;
    }

    // Output the result
    println!("{}", product);
}
