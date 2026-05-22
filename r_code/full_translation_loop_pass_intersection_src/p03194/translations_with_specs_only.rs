use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut p: i64 = it.next().unwrap().parse().unwrap();

    // Specification preconditions: n >= 1, p >= 1. Behavior otherwise unspecified.
    let mut ans: i64 = 1;

    let mut d: i64 = 2;
    while d * d <= p {
        if p % d == 0 {
            let mut e: i64 = 0;
            while p % d == 0 {
                p /= d;
                e += 1;
            }
            let k = e / n;
            for _ in 0..k {
                ans *= d;
            }
        }
        d += 1;
    }

    // If remaining p > 1, it is a prime factor with exponent 1 in the remaining value.
    // In general, after trial division, remaining p is either 1 or prime with exponent 1
    // relative to the remaining factorization; but if original p had a prime factor > sqrt(original p),
    // it will appear here with its remaining exponent (which must be 1 after removing smaller factors).
    if p > 1 {
        // Remaining exponent is 1, so contributes only if n == 1.
        if n == 1 {
            ans *= p;
        }
    }

    println!("{ans}");
}
