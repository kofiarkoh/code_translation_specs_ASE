use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    let h: i64 = m / 2; // truncation toward zero, as in Rust for signed integers

    let s: i64 = if m == 1 {
        0
    } else if h < n {
        h
    } else if h - n < 2 {
        n
    } else {
        n + (h - n) / 2 // truncation toward zero
    };

    print!("{}", s);
}
