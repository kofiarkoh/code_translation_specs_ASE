use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse three "long double" values.
    // Rust has no long double; use f64 as the narrowest standard choice.
    let mut it = input.split_whitespace();
    let a: f64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let b: f64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let c: f64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // The specification mentions "adjusted by a specific mathematical condition"
    // but does not define it. The narrowest reasonable interpretation is:
    // determine whether (a + b) < c.
    let ans = if a + b < c { "Yes" } else { "No" };

    print!("{ans}");
}
