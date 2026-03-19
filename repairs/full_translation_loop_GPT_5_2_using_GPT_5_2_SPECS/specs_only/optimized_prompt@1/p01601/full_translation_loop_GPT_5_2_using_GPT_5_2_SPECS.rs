use std::io::{self, Read};

fn is_decimal_palindrome_u128(mut x: u128) -> bool {
    // Decimal palindrome check by reversing digits.
    let orig = x;
    let mut rev: u128 = 0;
    while x > 0 {
        rev = rev * 10 + (x % 10);
        x /= 10;
    }
    orig == rev
}

fn main() {
    // No explicit error handling per spec: assume valid integer input exists.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap();
    let n: i128 = s.parse().unwrap();

    // Search outward with non-decreasing distance; tie breaks toward smaller value.
    // We restrict candidates to non-negative integers.
    let mut d: u128 = 0;
    loop {
        // Check smaller candidate first to satisfy tie-breaking toward smaller.
        if n >= 0 {
            let nn = n as u128;
            if d <= nn {
                let down = nn - d;
                if is_decimal_palindrome_u128(down) {
                    println!("{}", down);
                    return;
                }
            }
        } else {
            // For negative n, the "down" side is always negative, so no non-negative candidate.
        }

        // Check larger candidate.
        // Per spec, behavior is undefined if required arithmetic overflows; we use checked add
        // and, if it overflows, we loop forever (non-termination is allowed by spec).
        let up_opt: Option<u128> = if n >= 0 {
            let nn = n as u128;
            nn.checked_add(d)
        } else {
            // n < 0: up = d - (-n) when d >= -n, else negative (not allowed).
            let neg = (-n) as u128;
            if d >= neg {
                Some(d - neg)
            } else {
                None
            }
        };

        if let Some(up) = up_opt {
            if is_decimal_palindrome_u128(up) {
                println!("{}", up);
                return;
            }
        } else {
            // If we cannot form a non-negative "up" candidate yet, keep increasing d.
        }

        d = d.wrapping_add(1);
        // If d wraps, behavior is not defined by spec; wrapping keeps code total.
    }
}
