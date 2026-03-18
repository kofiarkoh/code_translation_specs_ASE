use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse up to five ints; if fewer are present, default to 0.
    // This approximates C behavior in the sense that uninitialized locals are UB in C,
    // but for well-formed inputs (five ints) it matches exactly.
    let mut it = input.split_whitespace();
    let a: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let c: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let d: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let e: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

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
