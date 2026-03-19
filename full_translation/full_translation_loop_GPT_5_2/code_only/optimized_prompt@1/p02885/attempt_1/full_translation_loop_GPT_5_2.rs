use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf("%d %d", ...) ignores leading whitespace).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // If parsing fails (e.g., missing numbers), C's scanf would leave variables unchanged (0 here).
    let mut it = input.split_whitespace();
    let a: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // In C, signed overflow is undefined; to emulate typical two's-complement behavior,
    // use wrapping arithmetic.
    let expr = a.wrapping_sub(b.wrapping_mul(2));
    let out = if expr > 0 { expr } else { 0 };

    // Match printf("%d", ...) exactly: no trailing newline.
    print!("{}", out);
}
