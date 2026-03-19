use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: usize = it.next().unwrap().parse().unwrap();

    // Specification mentions a "counting array" with indexable range, but does not define bounds.
    // Narrowest faithful interpretation without inventing bounds: count occurrences via a map,
    // then sort the resulting counts (equivalent to sorting non-zero entries of a counting array).
    use std::collections::HashMap;
    let mut freq: HashMap<i64, i64> = HashMap::new();

    for _ in 0..n {
        if let Some(tok) = it.next() {
            let x: i64 = tok.parse().unwrap();
            *freq.entry(x).or_insert(0) += 1;
        } else {
            // If fewer than N integers are provided, behavior is unspecified; stop reading.
            break;
        }
    }

    let mut counts: Vec<i64> = freq.values().copied().collect();
    counts.sort(); // non-decreasing

    // Compute sum of the counts of the first (N - K) unique integers.
    // If there are fewer than (N - K) unique integers, output zero.
    let need = n.saturating_sub(k);
    let ans: i64 = if counts.len() < need {
        0
    } else {
        counts.iter().take(need).sum()
    };

    println!("{}", ans);
}
