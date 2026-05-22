use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse three signed 64-bit integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions (per spec): d > 0, k >= 0, and all intermediate arithmetic fits in i64.
    // No explicit error handling is required; unwraps are acceptable under the spec.

    // a = |x|
    // Use i64::abs() which is defined for all values except i64::MIN; spec implies inputs
    // and intermediates are within range, so we rely on that precondition.
    let a: i64 = x.abs();

    // q = floor(a / d)
    let q: i64 = a / d;

    // t = min(k, q)
    let t: i64 = if k < q { k } else { q };

    // r = a - t * d
    let r: i64 = a - t * d;

    // If k > q and (k - q) is odd => result = |r - d| else r
    let result: i64 = if k > q && ((k - q) & 1) == 1 {
        (r - d).abs()
    } else {
        r
    };

    // Output is a single non-negative integer.
    println!("{}", result);
}
