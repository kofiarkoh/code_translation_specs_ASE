use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical scanf usage; if parsing fails, do nothing like C UB-ish cases).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // In C, failing scanf leaves variables uninitialized (undefined behavior).
    // In Rust we must initialize; if any value is missing/unparseable, we exit without output.
    let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let d: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let e: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if a == 0 {
        print!("1");
    } else if b == 0 {
        print!("2");
    } else if c == 0 {
        print!("3");
    } else if d == 0 {
        print!("4");
    } else if e == 0 {
        print!("5");
    }
}
