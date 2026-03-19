use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    // If input is missing/invalid, C's scanf would leave variables uninitialized (UB).
    // Here we default to 0 to keep the program defined in Rust.
    let a: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let c: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    if a == 1 && b == 0 && c == 0 {
        print!("Close\n");
    } else if a == 0 && b == 1 && c == 0 {
        print!("Close\n");
    } else if a == 1 && b == 1 && c == 0 {
        print!("Open\n");
    } else if a == 0 && b == 0 && c == 1 {
        print!("Open\n");
    } else if a == 0 && b == 0 && c == 0 {
        print!("Close\n");
    }
}
