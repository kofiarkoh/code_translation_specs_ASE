use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // SPEC: For each j in 1..=N, choose s_j as the smallest power of two such that k <= s_j * j.
    // Then output sum_{j=1..N} 1/s_j with 12 digits after decimal.
    //
    // Note: The provided C code does not implement this correctly:
    // - It uses a single `multi` shared across all i, making s_j non-minimal for smaller j.
    // - It also mistakenly uses `multi * N` instead of `multi * i` when storing denominators.
    // Per instruction, we follow the SPEC as source-of-truth.

    let mut sum: f64 = 0.0;

    for j in 1..=n {
        // Find minimal power-of-two s such that k <= s * j.
        // Preconditions say overflow does not occur; we still use i128 for headroom.
        let mut s: i128 = 1;
        let jj: i128 = j as i128;
        let kk: i128 = k as i128;

        while kk > s * jj {
            s *= 2;
        }

        sum += 1.0 / (s as f64);
    }

    print!("{:.12}", sum);
}
