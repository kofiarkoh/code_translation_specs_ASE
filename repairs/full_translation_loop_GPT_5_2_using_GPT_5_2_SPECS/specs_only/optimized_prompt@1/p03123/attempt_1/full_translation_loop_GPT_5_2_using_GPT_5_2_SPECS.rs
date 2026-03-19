use std::io::{self, Read};

fn main() {
    // Specification is incomplete: it references an "internal feasibility model",
    // a "fixed large prime constant", "hardcoded search ranges", and even possible
    // uninitialized reads on some paths. None of those concrete details (predicate,
    // prime value, ranges, arithmetic) are provided, so the exact semantics cannot
    // be reconstructed without inventing behavior, which is disallowed.
    //
    // Narrowest reasonable interpretation that preserves only the explicitly stated
    // observable I/O behavior:
    // - Read two i64 values (red, blue).
    // - If no positive k is feasible, print -1.
    //
    // With no feasibility predicate defined, we cannot deem any positive k feasible.
    // Therefore, the only output we can justify from the specification alone is -1.
    //
    // We still perform input parsing as specified; malformed input behavior is
    // unspecified, so we simply return without output on parse failure.

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();
    let red: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => return,
    };
    let blue: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => return,
    };

    // Inputs are "intended to be nonnegative", but no explicit handling is specified.
    // We do not add extra behavior based on sign.
    let _ = (red, blue);

    println!("-1");
}
