// knapsack.rs
//
// Translated from the provided C program according to the specification.
// Behavior:
//  - Repeatedly read W (capacity). If W == 0 -> terminate.
//  - Read N (number of items).
//  - Read N items in the "value,weight" format (value first, then weight).
//  - Solve 0/1 knapsack (each item at most once) to maximize total value with total weight <= W.
//  - Among selections achieving the maximum value, choose the one with minimal total weight.
//  - Print for each case:
//      Case X:
//      <max_value>
//      <min_weight>
//
// Implementation notes:
//  - Uses a standard 1-D DP (weight-down iteration) with Option<i32> to represent reachability.
//    This preserves correct behavior even if negative values appear (the original C code used -1 sentinel).
//  - Input parsing is tolerant of whitespace; item tokens are expected in the exact "value,weight" form per spec.
//  - Output formatting matches the spec exactly.

use std::io::{self, Read};

fn parse_items_token(token: &str) -> Option<(i32, usize)> {
    // token expected as "value,weight" with a literal comma.
    let mut parts = token.split(',');
    let v_str = parts.next()?;
    let w_str = parts.next()?;
    // If there are extra parts, treat as parse error.
    if parts.next().is_some() {
        return None;
    }
    let value = v_str.parse::<i32>().ok()?;
    let weight_i32 = w_str.parse::<i32>().ok()?;
    // weight must be non-negative to index dp; negative weights are undefined by spec.
    if weight_i32 < 0 {
        return None;
    }
    Some((value, weight_i32 as usize))
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0usize;
    let mut case_no = 0usize;

    // Helper to get next token
    let mut next_token = || -> Option<&'static str> {
        // This closure can't capture tokens by reference if returning &'static str,
        // so instead we'll not use this closure. We'll use inline checks below.
        None
    };

    // We'll iterate using idx over tokens vector.
    while idx < tokens.len() {
        // Read W
        let w_token = tokens[idx];
        idx += 1;
        // Try parse W as i32
        let W = match w_token.parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                // Malformed input: per spec behavior is undefined; just stop.
                break;
            }
        };
        if W == 0 {
            // Termination sentinel: do not process further cases.
            break;
        }
        if idx >= tokens.len() {
            // No more tokens for N; malformed input.
            break;
        }
        // Read N
        let n_token = tokens[idx];
        idx += 1;
        let N = match n_token.parse::<i32>() {
            Ok(x) => x,
            Err(_) => break,
        };
        if N < 0 {
            // Per spec N is expected non-negative; treat malformed by stopping.
            break;
        }
        let N_usize = N as usize;

        // Read N items tokens
        let mut items: Vec<(i32, usize)> = Vec::with_capacity(N_usize);
        for _ in 0..N_usize {
            if idx >= tokens.len() {
                // malformed input
                break;
            }
            let tok = tokens[idx];
            idx += 1;
            match parse_items_token(tok) {
                Some(it) => items.push(it),
                None => {
                    // Malformed token; per spec this is undefined; stop processing.
                    // We'll terminate program.
                    return;
                }
            }
        }
        if items.len() != N_usize {
            // insufficient items; stop.
            break;
        }

        // Capacity W is non-negative per spec
        if W < 0 {
            // undefined per spec; stop
            break;
        }
        let capacity = W as usize;

        // DP: dp[w] = Option<max_value achievable with exact weight w>
        // Initialize dp[0] = Some(0)
        let mut dp: Vec<Option<i32>> = vec![None; capacity + 1];
        dp[0] = Some(0);

        for &(value, weight) in &items {
            if weight > capacity {
                // Item too heavy to be included in any feasible weight <= W; skip
                continue;
            }
            // iterate weights downwards for 0/1 knapsack
            for cur_w in (weight..=capacity).rev() {
                if let Some(prev) = dp[cur_w - weight] {
                    let cand = prev.saturating_add(value);
                    match dp[cur_w] {
                        Some(existing) => {
                            if cand > existing {
                                dp[cur_w] = Some(cand);
                            }
                        }
                        None => {
                            dp[cur_w] = Some(cand);
                        }
                    }
                }
            }
        }

        // Find maximum value among reachable weights 0..=capacity
        // and minimal weight achieving that maximum.
        let mut max_value_opt: Option<i32> = None;
        let mut min_weight_for_max: usize = 0;
        for w in 0..=capacity {
            if let Some(v) = dp[w] {
                match max_value_opt {
                    Some(maxv) => {
                        if v > maxv {
                            max_value_opt = Some(v);
                            min_weight_for_max = w;
                        } else if v == maxv {
                            if w < min_weight_for_max {
                                min_weight_for_max = w;
                            }
                        }
                    }
                    None => {
                        max_value_opt = Some(v);
                        min_weight_for_max = w;
                    }
                }
            }
        }

        // Per spec the empty selection is always valid, so ensure at least 0,0
        let (max_value, min_weight) = match max_value_opt {
            Some(v) => (v, min_weight_for_max),
            None => (0, 0), // should not happen because dp[0]=Some(0)
        };

        case_no += 1;
        // Print output exactly as specified
        println!("Case {}:", case_no);
        println!("{}", max_value);
        println!("{}", min_weight);
    }
}
