use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style of the C code).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Spec precondition: m >= 1. The C code does not enforce it; we assume valid input.
    let sum: i64;
    if m == 1 {
        sum = 0;
    } else {
        let half = m / 2; // C integer division truncates toward zero; for m>=1 this matches Rust.
        let diff = half - n;
        if diff < 0 {
            sum = half;
        } else if diff < 2 {
            sum = n;
        } else {
            sum = n + diff / 2;
        }
    }

    // Preserve exact output formatting: no trailing newline.
    print!("{sum}");
}
