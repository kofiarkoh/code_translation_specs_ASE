use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    // The judge input consists of multiple test cases:
    // each case is: <target> <string>, terminated by a single "0" target.
    // This is inferred from the provided sample I/O.
    let mut outputs: Vec<String> = Vec::new();

    loop {
        let t_tok = match it.next() {
            Some(x) => x,
            None => break,
        };
        let target: i64 = match t_tok.parse() {
            Ok(v) => v,
            Err(_) => {
                // Narrowest reasonable behavior for malformed input.
                outputs.push("error".to_string());
                break;
            }
        };
        if target == 0 {
            break;
        }
        let s = match it.next() {
            Some(x) => x.to_string(),
            None => {
                outputs.push("error".to_string());
                break;
            }
        };

        outputs.push(solve_case(target, &s));
    }

    print!("{}", outputs.join("\n"));
}

fn solve_case(target: i64, s: &str) -> String {
    // Exceptional behavior / preconditions.
    if target <= 0 || s.is_empty() || s.len() > 10 {
        return "error".to_string();
    }

    // Interpret "substrings" as contiguous substrings, and "combination"
    // as a partition of the whole string into contiguous pieces.
    //
    // For each partition, derive integer values from each piece (base-10),
    // and compute their sum. Only consider partitions with sum <= target.
    // Choose the maximum sum; if multiple partitions achieve it => rejected.
    //
    // Output format (from sample): "<sum> <piece1> <piece2> ...".
    let bytes = s.as_bytes();
    let n = bytes.len();

    // Precompute numeric value of every substring s[i..j) if all digits and fits i64.
    let mut val = vec![vec![None::<i64>; n + 1]; n + 1];
    for i in 0..n {
        let mut acc: i64 = 0;
        let mut ok = true;
        for j in i + 1..=n {
            let ch = bytes[j - 1];
            if !(b'0'..=b'9').contains(&ch) {
                ok = false;
            }
            if ok {
                let digit = (ch - b'0') as i64;
                match acc.checked_mul(10).and_then(|x| x.checked_add(digit)) {
                    Some(v) => acc = v,
                    None => ok = false,
                }
            }
            if ok {
                val[i][j] = Some(acc);
            } else {
                val[i][j] = None;
            }
        }
    }

    let mut best_sum: i64 = -1;
    let mut best_parts: Vec<String> = Vec::new();
    let mut best_count: usize = 0;

    // Enumerate all partitions via bitmask of cut positions between characters.
    // There are (n-1) possible cut points.
    let cuts = if n >= 2 { n - 1 } else { 0 };
    let total_masks = 1u32 << cuts;

    for mask in 0..total_masks {
        let mut parts: Vec<(usize, usize)> = Vec::new();
        let mut start = 0usize;
        for i in 0..cuts {
            if (mask >> i) & 1 == 1 {
                let end = i + 1;
                parts.push((start, end));
                start = end;
            }
        }
        parts.push((start, n));

        let mut sum: i64 = 0;
        let mut ok = true;
        for &(a, b) in &parts {
            match val[a][b] {
                Some(v) => {
                    // Sum is non-negative; still guard overflow.
                    match sum.checked_add(v) {
                        Some(ns) => sum = ns,
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok || sum > target {
            continue;
        }

        if sum > best_sum {
            best_sum = sum;
            best_count = 1;
            best_parts = parts
                .iter()
                .map(|&(a, b)| s[a..b].to_string())
                .collect();
        } else if sum == best_sum {
            best_count += 1;
        }
    }

    if best_sum < 0 {
        "error".to_string()
    } else if best_count > 1 {
        "rejected".to_string()
    } else {
        let mut out = best_sum.to_string();
        for p in best_parts {
            out.push(' ');
            out.push_str(&p);
        }
        out
    }
}
