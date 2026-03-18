use std::collections::HashMap;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let s_low = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };
    let s_high = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };
    let s_target = match it.next() {
        Some(s) => s.to_string(),
        None => process::exit(1),
    };

    // Undefined behavior for invalid characters: we unwrap digit conversion.
    let low: Vec<u8> = s_low
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();
    let high: Vec<u8> = s_high
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();
    let target: Vec<u8> = s_target
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect();

    // Semantics inferred from the spec + sample:
    // Count the number of sequences X of length = target.len() such that:
    // - For each i, low[i] <= X[i] <= high[i] (bounds are per-position digits).
    // - X is lexicographically <= target (i.e., as a digit string).
    //
    // This matches sample: low=123, high=456, target=12 (length 2)
    // => bounds per position: [1..4], [2..5], count of 2-digit numbers <= 12 is 10.
    //
    // If bounds are shorter than target, missing positions default to [0..9].
    // If bounds are longer than target, extra digits are ignored (only positions in target matter).
    let mut memo: HashMap<(usize, bool), u64> = HashMap::new();
    let ans = dp(0, true, &low, &high, &target, &mut memo);
    println!("{ans}");
}

fn dp(
    pos: usize,
    tight: bool,
    low: &[u8],
    high: &[u8],
    target: &[u8],
    memo: &mut HashMap<(usize, bool), u64>,
) -> u64 {
    if pos == target.len() {
        return 1;
    }
    if let Some(&v) = memo.get(&(pos, tight)) {
        return v;
    }

    let lo = if pos < low.len() { low[pos] } else { 0 };
    let hi = if pos < high.len() { high[pos] } else { 9 };

    let limit = if tight { target[pos] } else { 9 };
    let upper = hi.min(limit);

    let mut sum: u64 = 0;
    if lo <= upper {
        for d in lo..=upper {
            let next_tight = tight && d == limit;
            sum = sum.wrapping_add(dp(pos + 1, next_tight, low, high, target, memo));
        }
    }

    memo.insert((pos, tight), sum);
    sum
}
