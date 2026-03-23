use std::io::{self, Read};

fn main() {
    // Match C string literals and puts() behavior (prints with trailing newline).
    const A: &str = "Alice";
    const B: &str = "Bob";
    const D: &str = "Draw";

    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // Emulate `scanf("%d%d%d%d", ...) != 4`:
    // if fewer than 4 integers are successfully parsed, exit with code 1 and no output.
    let t1: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let t2: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let r1: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let r2: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let out = if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            A
        } else if t1 > t2 {
            B
        } else {
            D
        }
    } else {
        if r1 > r2 {
            A
        } else if r1 < r2 {
            B
        } else {
            D
        }
    };

    println!("{out}");
}
