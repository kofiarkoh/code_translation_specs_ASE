use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().expect("Invalid input for x");
    let d: i64 = iter.next().unwrap().parse().expect("Invalid input for d");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Preconditions: d must be non-zero
    if d == 0 {
        panic!("Divisor cannot be zero");
    }

    let mut now = x.abs(); // Take the absolute value of x
    let mut k = k; // Make a mutable copy of k

    if now / d <= k {
        now -= d * (now / d);
        k -= now / d;
        if k % 2 != 0 {
            now -= d;
        }
    } else {
        now -= d * k;
    }

    // Output the absolute value of now
    println!("{}", now.abs());
}
