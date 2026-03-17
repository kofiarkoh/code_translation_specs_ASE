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

    let n = (n % 12).rem_euclid(12); // Ensure N is in the range [0, 11]
    let mut i = 0;

    while i < n {
        a -= b;
        i += 1;
        if i == n {
            break;
        }
        b += a;
        i += 1;
    }

    // Output the results
    println!("{} {}", a, b);
}
