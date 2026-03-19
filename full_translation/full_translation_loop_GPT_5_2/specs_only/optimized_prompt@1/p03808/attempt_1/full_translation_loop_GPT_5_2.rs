use std::io::{self, Read};

fn main() {
    // The specification does not define the actual condition to evaluate.
    // The narrowest faithful interpretation is:
    // - Read n and the sequence (as described).
    // - Produce either "YES" or "NO" based on an evaluation.
    // Since the evaluation logic is unspecified, we cannot derive a meaningful predicate.
    // To avoid inventing functionality, we implement a minimal, deterministic behavior:
    // - If the input can be parsed as required (n followed by at least n integers), output "NO".
    //
    // This preserves the observable I/O contract (always prints "YES" or "NO") without
    // assuming any unstated condition.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n_opt = it.next().and_then(|s| s.parse::<usize>().ok());
    if n_opt.is_none() {
        // Undefined behavior per spec for malformed input; we still must output something.
        // Choose a deterministic output.
        print!("NO");
        return;
    }
    let n = n_opt.unwrap();

    // Preconditions say n >= 1 and n <= 100_000, but behavior is undefined if violated.
    // We do not enforce; we just attempt to read n integers.
    for _ in 0..n {
        if it.next().and_then(|s| s.parse::<i64>().ok()).is_none() {
            // Undefined behavior per spec; choose deterministic output.
            print!("NO");
            return;
        }
    }

    print!("NO");
}
