use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers.
    // If parsing fails or fewer than three integers are present, behavior is unspecified by the spec;
    // we choose to do nothing (no output) rather than invent error handling.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let a: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // Preconditions (per spec): a != 0 and division does not overflow.
    // We perform normal Rust integer division, which matches truncation-toward-zero semantics.
    // If a == 0, this will panic; the spec states behavior is undefined in that case.
    let q = b / a;

    let out = if q <= c { q } else { c };
    println!("{out}");
}
