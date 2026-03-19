use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Spec: h = floor(m/2) with truncation toward zero (Rust i64 division matches C for /).
    let h = m / 2;

    let s: i64 = if m == 1 {
        0
    } else if h < n {
        h
    } else if h - n < 2 {
        n
    } else {
        n + (h - n) / 2
    };

    // Preserve I/O formatting: print the integer with no trailing newline (as in C code).
    print!("{s}");
}
