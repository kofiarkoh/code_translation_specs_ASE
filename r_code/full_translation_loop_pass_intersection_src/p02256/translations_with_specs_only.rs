use std::io::{self, Read};
use std::process;

fn gcd_euclid(mut a: u64, mut b: u64) -> u64 {
    // Euclidean algorithm: (a, b) := (b, a mod b) until remainder is zero.
    // Additional specified behavior: if divisor becomes zero, stop and return current divisor.
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    // Read entire stdin; malformed/missing input is unspecified by the spec,
    // so we do not add explicit error handling beyond what parsing naturally does.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a_str = it.next().unwrap();
    let b_str = it.next().unwrap();

    let a: i64 = a_str.parse().unwrap();
    let b: i64 = b_str.parse().unwrap();

    // Range validation: inclusive [1, 1_000_000_000]
    const LO: i64 = 1;
    const HI: i64 = 1_000_000_000;

    if a < LO || a > HI || b < LO || b > HI {
        // No output on failure.
        process::exit(1);
    }

    // Order so first is larger (or equal), second is smaller (or equal).
    let (mut x, mut y) = (a as u64, b as u64);
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }

    let g = gcd_euclid(x, y);
    println!("{g}");
    process::exit(0);
}
