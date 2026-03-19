use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Specification: behavior is undefined on overflow.
    // In Rust, debug builds panic on overflow; release builds wrap.
    // We use wrapping ops to avoid introducing panics and keep behavior
    // as close as possible to "not defined" without adding error handling.
    let diff = a.wrapping_sub(b.wrapping_mul(2));

    let out = if diff < 0 { 0 } else { diff };
    print!("{out}");
}
