use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut it = input.split_whitespace();

    let t1: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return, // fewer than four integers => no output
    };
    let t2: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let r1: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let r2: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // Primary comparison: time values.
    // If time values differ, decide winner solely by time.
    // If time values are equal, compare result values only if both are non-negative.
    // If any result value is negative, still output a result based on time values;
    // since times are equal here, that implies "Draw".
    let out = if t1 < t2 {
        "Alice"
    } else if t1 > t2 {
        "Bob"
    } else {
        if r1 >= 0 && r2 >= 0 {
            if r1 > r2 {
                "Alice"
            } else if r1 < r2 {
                "Bob"
            } else {
                "Draw"
            }
        } else {
            "Draw"
        }
    };

    println!("{out}");
}
