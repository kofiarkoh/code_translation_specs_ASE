use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Per spec, behavior is undefined if missing/invalid/non-positive; we keep it simple.
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Fixed memory limit mentioned: up to 1000 integers. Spec doesn't define behavior beyond that.
    // We do not enforce; we just read n integers as required.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // Iteratively sort by remainder for each divisor from 1 to m.
    // To preserve the invariant that previous divisor ordering remains consistent,
    // we use a stable sort at each step.
    //
    // Remainder semantics: use Rust's `%` on i64, which matches typical C remainder behavior
    // (truncation toward zero; remainder has the sign of the dividend).
    for d in 1..=m {
        a.sort_by_key(|&x| x % d);
    }

    // Output each integer on its own line.
    let mut out = String::new();
    for x in a {
        out.push_str(&format!("{x}\n"));
    }
    print!("{out}");
}
