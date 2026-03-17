// AOJ 3130 Calculation Training
// 2020.1.14 bal4u

use std::io::{self, Write};

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse inputs
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let mut a: i32 = iter.next().unwrap().parse().expect("Invalid input for A");
    let mut b: i32 = iter.next().unwrap().parse().expect("Invalid input for B");

    // Calculate effective iterations
    let n = (n % 12).max(0); // Ensure N is non-negative

    // Perform calculations
    for i in 0..n {
        a -= b; // A = A - B
        if i + 1 < n {
            b += a; // B = A + B
        }
    }

    // Output the final values of A and B
    println!("{} {}", a, b);
}
