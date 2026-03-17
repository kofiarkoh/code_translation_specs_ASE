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

    // Preconditions check
    if n < 0 {
        panic!("N must be a non-negative long integer");
    }

    // Calculate the number of iterations based on N modulo 12
    let n = (n % 12) as usize;
    let mut i = 0;

    // Iterative process
    while i < n {
        a -= b; // A = A - B
        i += 1;
        if i == n {
            break;
        }
        b += a; // B = A + B
        i += 1;
    }

    // Output the final values of A and B
    println!("{} {}", a, b);
}
