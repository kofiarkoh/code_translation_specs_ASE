use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf("%d%d", ...) ignores whitespace similarly).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // If input is missing, C's scanf would leave variables uninitialized (UB).
    // We default to 0 here to keep the program total in Rust.
    let a: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // In C, signed overflow is undefined behavior. To emulate common two's-complement
    // wraparound behavior, use wrapping arithmetic.
    let c = a.wrapping_add(b);
    let d = a.wrapping_sub(b);
    let e = a.wrapping_mul(b);

    let mut max: i32 = -1000;
    if max < c {
        max = c;
    }
    if max < d {
        max = d;
    }
    if max < e {
        max = e;
    }

    // Match C's printf("%d", max) (no trailing newline).
    print!("{}", max);
}
