use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C scanf("%d") would (whitespace-separated integers).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };
    let a_lim: i32 = it.next().unwrap().parse().unwrap();
    let b_lim: i32 = it.next().unwrap().parse().unwrap();

    let mut count: i32 = 0;

    for i in 1..=n {
        // Mirror the C digit extraction exactly (including behavior for i > 99999).
        let e = i % 10;
        let d = ((i - e) / 10) % 10;
        let c = ((i - 10 * d - e) / 100) % 10;
        let b = ((i - 100 * c - 10 * d - e) / 1000) % 10;
        let a = ((i - 1000 * b - 100 * c - 10 * d - e) / 10000) % 10;

        let sum = a + b + c + d + e;
        if a_lim <= sum && sum <= b_lim {
            // In C, signed overflow is UB; typical contest constraints avoid it.
            // Use wrapping_add to avoid panics in Rust debug builds and approximate
            // common two's-complement behavior if overflow occurs.
            count = count.wrapping_add(i);
        }
    }

    print!("{}", count);
}
