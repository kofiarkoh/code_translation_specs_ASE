use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return; // No output on input failure.
    }

    let mut it = input.split_whitespace();
    let t1 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return, // Fewer than four integers => terminate without output (per spec).
    };
    let t2 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let r1 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let r2 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };

    let out = if r1 < 0 || r2 < 0 {
        // If any result is negative, decide by time values.
        if t1 < t2 {
            "Alice"
        } else if t1 > t2 {
            "Bob"
        } else {
            "Draw"
        }
    } else {
        // Otherwise decide by result values.
        if r1 > r2 {
            "Alice"
        } else if r1 < r2 {
            "Bob"
        } else {
            "Draw"
        }
    };

    println!("{out}");
}
