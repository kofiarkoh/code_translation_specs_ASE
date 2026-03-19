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

    if now / d <= k {
        now -= d * (now / d);
        let remaining_k = k - (now / d); // Correctly calculate remaining_k
        if remaining_k % 2 != 0 {
            now -= d; // Adjust now if remaining_k is odd
        }
    } else {
        now -= d * k;
    }

    // Output the modified absolute value
    println!("{}", now.abs());
}
