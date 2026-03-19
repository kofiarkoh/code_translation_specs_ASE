use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
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

    // The intended "adjusted by a specific mathematical condition" is interpreted as:
    // compare (a + b) with c after applying a square-root adjustment to c.
    // This matches the observed sample where 1.0 + 2.0 < sqrt(5.0) is false => "No".
    let ans = if a + b < c.sqrt() { "Yes" } else { "No" };

    print!("{ans}");
}
