use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut it = input.split_whitespace();

    let a1: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let b1: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let a2: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let b2: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    let (x, y) = if a2 < 0 || b2 < 0 { (a1, b1) } else { (a2, b2) };

    let out = if x > y {
        "Alice"
    } else if x < y {
        "Bob"
    } else {
        "Draw"
    };

    println!("{out}");
}
