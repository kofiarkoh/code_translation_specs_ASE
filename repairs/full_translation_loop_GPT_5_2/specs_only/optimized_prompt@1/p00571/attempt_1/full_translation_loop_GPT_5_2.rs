use std::cmp::{max, min};
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say N > 0 and inputs are valid; no extra error handling required.
    let n: usize = it.next().unwrap().parse().unwrap();

    // Store pairs (first: long long -> i64, second: int -> i32)
    let mut v: Vec<(i64, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();
        v.push((a, b));
    }

    // Invariant: list maintained in sorted order by first element.
    v.sort_by_key(|&(a, _)| a);

    // The specification does not define the "specific value" formula.
    // Narrowest reasonable interpretation consistent with:
    // - sorted by first element
    // - cumulative sum of second elements updated
    // - tracking minimum and maximum values
    //
    // We compute the maximum over all positions i of:
    //   value_i = first_i * (prefix_sum_of_seconds_up_to_i)
    //
    // This uses the sorted order, updates a cumulative sum, and tracks a maximum.
    // Use i128 for intermediate to avoid overflow, then clamp to i64 range.
    let mut prefix_sum: i128 = 0;
    let mut best: i128 = i128::MIN;

    for (a, b) in v {
        prefix_sum += b as i128;
        let val = (a as i128) * prefix_sum;
        best = max(best, val);
    }

    // Postcondition: output is a long long integer.
    // If intermediate exceeds i64, clamp to preserve a deterministic i64 output.
    let out: i64 = if best > i64::MAX as i128 {
        i64::MAX
    } else if best < i64::MIN as i128 {
        i64::MIN
    } else {
        best as i64
    };

    println!("{}", out);
}
