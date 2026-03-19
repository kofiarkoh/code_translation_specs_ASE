use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // Read entire stdin and split by whitespace into tokens (strings contain no whitespace).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Sizes are specified as non-negative; no explicit error handling is required by spec.
    let n1: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => {
            // No input; narrowest behavior: produce no output (spec doesn't define this case).
            return;
        }
    };

    // Count occurrences in first list.
    let mut c1: HashMap<String, i32> = HashMap::new();
    for _ in 0..n1 {
        if let Some(s) = it.next() {
            *c1.entry(s.to_string()).or_insert(0) += 1;
        } else {
            // Incomplete input; spec provides no behavior. Stop consuming.
            break;
        }
    }

    let n2: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => {
            // If second size missing, treat as no second list (n2 = 0) per narrowest interpretation.
            // Compute result based on first list only.
            let mut best: i32 = 0;
            for &v in c1.values() {
                if v > best {
                    best = v;
                }
            }
            println!("{}", best.max(0));
            return;
        }
    };

    // Count occurrences in second list.
    let mut c2: HashMap<String, i32> = HashMap::new();
    for _ in 0..n2 {
        if let Some(s) = it.next() {
            *c2.entry(s.to_string()).or_insert(0) += 1;
        } else {
            break;
        }
    }

    // Compute max over distinct strings appearing in the first list of (c1 - c2), clamped at 0.
    let mut best: i32 = 0;
    for (k, &v1) in c1.iter() {
        let v2 = c2.get(k).copied().unwrap_or(0);
        let diff = v1 - v2;
        if diff > best {
            best = diff;
        }
    }

    if best < 0 {
        best = 0;
    }
    println!("{}", best);
}
