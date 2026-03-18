use std::io::{self, Write};

fn main() {
    const A: &str = "Alice";
    const B: &str = "Bob";
    const D: &str = "Draw";

    let mut input = String::new();
    // Read input from stdin
    if io::stdin().read_line(&mut input).is_err() {
        return; // Handle error in reading input
    }

    // Split the input into parts and parse them
    let mut iter = input.split_whitespace();
    let t1: i32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let t2: i32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let r1: i32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let r2: i32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Check if we have all four inputs
    if iter.count() != 0 {
        return; // Not enough inputs
    }

    // Logic to determine the output
    if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            println!("{}", A);
        } else if t1 > t2 {
            println!("{}", B);
        } else {
            println!("{}", D);
        }
    } else {
        if r1 > r2 {
            println!("{}", A);
        } else if r1 < r2 {
            println!("{}", B);
        } else {
            println!("{}", D);
        }
    }
}
