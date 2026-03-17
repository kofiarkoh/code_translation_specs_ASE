use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    if io::stdin().read_line(&mut input).is_err() {
        // Handle read error
        return;
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

    // Determine the output based on the conditions
    let output = if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            "Alice"
        } else if t1 > t2 {
            "Bob"
        } else {
            "Draw"
        }
    } else {
        if r1 > r2 {
            "Alice"
        } else if r1 < r2 {
            "Bob"
        } else {
            "Draw"
        }
    };

    // Print the result
    println!("{}", output);
}
