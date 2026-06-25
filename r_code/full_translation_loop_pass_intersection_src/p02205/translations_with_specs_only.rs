use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Specification: exactly three integers are provided; otherwise output is unspecified.
    // We keep parsing minimal and do not add extra behavior.
    let n: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let mut a: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let mut b: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Preconditions say N is non-negative. We compute k = N mod 12 in [0, 11].
    // Using Euclidean remainder to ensure the range even if input violates the precondition.
    let k: i64 = n.rem_euclid(12);

    // Apply exactly k transitions, with step index t = 1..k.
    for t in 1..=k {
        if t % 2 == 1 {
            // Odd t: (A, B) -> (A - B, B)
            a = a - b;
        } else {
            // Even t: (A, B) -> (A, A + B) using A_{t-1} + B_{t-1}
            b = a + b;
        }
    }

    println!("{} {}", a, b);
}
