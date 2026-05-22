use std::io::{self, Read};

fn main() {
    // Read entire stdin, mimicking scanf-style whitespace-separated parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // N is read as C `int` via `%d`. Preconditions say N is nonnegative.
    // If input is missing/malformed, behavior is undefined in the spec; we unwrap.
    let n_i32: i32 = it.next().unwrap().parse().unwrap();
    let n: usize = n_i32 as usize;

    // Read N values as C `long long` via `%lld`.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Sort nondecreasing.
    a.sort();

    // Traverse from largest to smallest, alternately adding to SumA then SumB.
    // The spec states overflow is undefined; Rust i64 overflow in debug would panic.
    // We use wrapping_add/sub to avoid panics and keep behavior well-defined in Rust,
    // while still matching the intended behavior under the spec's no-overflow precondition.
    let mut sum_a: i64 = 0;
    let mut sum_b: i64 = 0;

    for (k, &v) in a.iter().rev().enumerate() {
        if k % 2 == 0 {
            sum_a = sum_a.wrapping_add(v);
        } else {
            sum_b = sum_b.wrapping_add(v);
        }
    }

    let diff = sum_a.wrapping_sub(sum_b);
    print!("{}\n", diff);
}
