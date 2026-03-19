use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf("%d%d") ignores leading whitespace).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // If input is missing, C's scanf would leave variables uninitialized (UB).
    // We choose a deterministic fallback (0) rather than using unsafe uninitialized values.
    let a: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // In C, signed overflow is undefined; to emulate typical two's-complement wraparound,
    // use wrapping arithmetic.
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

    // Match C: no trailing newline.
    print!("{}", max);
}
