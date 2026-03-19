// AOJ 3130 Calculation Training
// 2020.1.14 bal4u

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut parts = input.split_whitespace();
    let n: i64 = parts.next().unwrap().parse().unwrap();
    let mut a: i32 = parts.next().unwrap().parse().unwrap();
    let mut b: i32 = parts.next().unwrap().parse().unwrap();

    let n = (n % 12) as usize; // Modulo operation to get N in the range [0, 11]
    let mut i = 0;

    while i < n {
        a -= b; // A = A - B
        i += 1;
        if i == n {
            break; // This break is redundant but kept for semantic preservation
        }
        b += a; // B = A + B
        i += 1;
    }

    // Print the results, preserving the format
    println!("{} {}", a, b);
}
