use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like behavior: whitespace-separated tokens)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code does not check scanf return value; if input is missing/invalid,
    // behavior is undefined. Here we unwrap to fail fast.
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // In C, (a+b) on long int can overflow (undefined behavior). Rust i64
    // would panic in debug on overflow; use wrapping to better emulate typical
    // two's-complement wraparound behavior.
    let sum = a.wrapping_add(b);

    if sum % 2 != 0 {
        print!("IMPOSSIBLE");
        return;
    }

    // C prints with "%d" (int) even though expression is long int; this is UB.
    // We emulate the common outcome: convert to 32-bit int and print as signed.
    let half = sum / 2;
    let as_i32 = half as i32;
    print!("{}", as_i32);
}
