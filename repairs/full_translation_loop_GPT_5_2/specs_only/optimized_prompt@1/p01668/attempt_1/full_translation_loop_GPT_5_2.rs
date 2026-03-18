use std::collections::HashMap;
use std::io::{self, Read};
use std::process;

/// Narrow, specification-faithful interpretation:
/// - Read three digit strings: lower, upper, target.
/// - Count sequences of length = target.len().
/// - For each position i:
///   - If i < lower.len(), digit >= lower[i], else digit >= 0.
///   - If i < upper.len(), digit <= upper[i], else digit <= 9.
///   - Additionally, digit must equal target[i] (i.e., "match the target sequence at certain positions"
///     is interpreted as "at all positions", the narrowest non-inventive constraint).
/// - Result is the number of valid sequences; with the equality constraint, this is 0 or 1.
/// - Memoization is included as required by the specification, even though the state space is small.
///
/// Undefined behavior for invalid (non-digit) characters is preserved by not validating beyond `to_digit(10)`
/// unwrap; this will panic on invalid input, which is acceptable given "behavior is undefined".
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let lower = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };
    let upper = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };
    let target = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };

    let lower_digits: Vec<u8> = lower
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();
    let upper_digits: Vec<u8> = upper
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();
    let target_digits: Vec<u8> = target
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();

    let mut memo: HashMap<usize, u64> = HashMap::new();
    let ans = dp_count(
        0,
        &lower_digits,
        &upper_digits,
        &target_digits,
        &mut memo,
    );

    println!("{ans}");
}

fn dp_count(
    pos: usize,
    lower: &[u8],
    upper: &[u8],
    target: &[u8],
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    if let Some(&v) = memo.get(&pos) {
        return v;
    }

    let res = if pos == target.len() {
        1
    } else {
        let lo = if pos < lower.len() { lower[pos] } else { 0 };
        let hi = if pos < upper.len() { upper[pos] } else { 9 };
        let d = target[pos];

        if d < lo || d > hi {
            0
        } else {
            dp_count(pos + 1, lower, upper, target, memo)
        }
    };

    memo.insert(pos, res);
    res
}
