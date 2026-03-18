use std::io::{self, Read};

fn is_prime(n: i32) -> bool {
    // Spec invariant: values used to determine primality are non-negative.
    // For n < 2, not prime.
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i: i32 = 3;
    while (i as i64) * (i as i64) <= n as i64 {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    // Read all integers from stdin; behavior on invalid input is unspecified (per spec),
    // so we simply ignore non-integer tokens by failing to parse them.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let l0: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => continue, // unspecified; choose to skip invalid tokens
        };

        // Find largest prime <= l0 by decrementing.
        let mut l = l0;
        while l >= 0 && !is_prime(l) {
            l -= 1;
        }

        // Find smallest prime >= l0 by incrementing.
        let mut u = l0;
        while u >= 0 && !is_prime(u) {
            u += 1;
        }

        // C prints u - l (difference between the two prime boundaries).
        // Note: This matches the C code, but the spec says "count of prime numbers between"
        // those boundaries; that would generally be different. We follow the C behavior.
        out.push_str(&(u - l).to_string());
        out.push('\n');
    }

    print!("{out}");
}
