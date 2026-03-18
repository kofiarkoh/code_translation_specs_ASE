use std::io::{self, Read};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Euclidean algorithm; valid inputs are positive, so gcd is well-defined.
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    // Read entire stdin; specification assumes two unsigned integers are provided.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // Narrow interpretation: if parsing fails or fewer than two numbers are present,
    // treat as invalid input and return failure status without output.
    let a: u64 = match it.next().and_then(|s| s.parse::<u64>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let b: u64 = match it.next().and_then(|s| s.parse::<u64>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Range check: 1..=1_000_000_000
    const MIN: u64 = 1;
    const MAX: u64 = 1_000_000_000;

    if !(MIN..=MAX).contains(&a) || !(MIN..=MAX).contains(&b) {
        // Invalid: do not output a GCD; return failure status.
        std::process::exit(1);
    }

    let g = gcd(a, b);
    println!("{g}");
    // Valid: return success status.
    std::process::exit(0);
}
