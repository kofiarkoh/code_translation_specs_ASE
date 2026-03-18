use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Narrowest reasonable interpretation:
    // If the first integer is missing, treat as n = 0 (no subsequent integers).
    let n: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap_or(0),
        None => 0,
    };

    // Store up to n integers; if fewer are provided, process only those available.
    // Use i64 as a "valid integer values" stand-in without assuming a narrower range.
    let mut values: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        match it.next() {
            Some(tok) => {
                // If parsing fails, stop consuming further values (spec doesn't define errors).
                if let Ok(v) = tok.parse::<i64>() {
                    values.push(v);
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    // Maintain a sorted collection to validate distinctness comparisons.
    values.sort_unstable();

    // Count distinct values in the sorted sequence.
    let mut distinct = 0usize;
    let mut prev: Option<i64> = None;
    for &v in &values {
        if prev.map_or(true, |p| p != v) {
            distinct += 1;
            prev = Some(v);
        }
    }

    println!("{}", distinct);
}
