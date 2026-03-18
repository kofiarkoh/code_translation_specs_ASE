use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Input format incorrect -> terminate with a return code (non-zero).
        std::process::exit(1);
    }

    let mut it = input.split_whitespace();

    // Parse n and w; if missing/invalid -> terminate with a return code (non-zero).
    let n: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let w_cap: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Preconditions: non-negative.
    if n < 0 || w_cap < 0 {
        std::process::exit(1);
    }
    let n = n as usize;
    let w_cap = w_cap as i64;

    // Read items; if missing/invalid -> terminate with a return code (non-zero).
    let mut items: Vec<(i64, i64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let wt: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        let val: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        // Specification does not state weights/values must be non-negative; keep as-is.
        items.push((wt, val));
    }

    // Fixed memory limit implied by a constant for maximum weight:
    // Use a value-based DP that tracks minimum weight for each achievable total value.
    // This keeps memory independent of W and bounded by sum of values.
    //
    // Narrow interpretation: values must be non-negative for this DP to be well-defined.
    // If any value is negative, we cannot index by total value; treat as invalid input format.
    let mut sum_values: i64 = 0;
    for &(_, v) in &items {
        if v < 0 {
            std::process::exit(1);
        }
        sum_values = match sum_values.checked_add(v) {
            Some(s) => s,
            None => std::process::exit(1),
        };
    }

    let sum_values_usize: usize = match usize::try_from(sum_values) {
        Ok(v) => v,
        Err(_) => std::process::exit(1),
    };

    // dp[val] = minimum weight needed to achieve total value == val.
    // Invariant: dp contains only valid (minimum) weights for each value, or INF if unreachable.
    let inf: i64 = i64::MAX / 4;
    let mut dp = vec![inf; sum_values_usize + 1];
    dp[0] = 0;

    for &(wt, val) in &items {
        let val_u: usize = val as usize;
        // 0/1 knapsack: iterate values descending to avoid reusing the same item.
        for v in (val_u..=sum_values_usize).rev() {
            let prev = dp[v - val_u];
            if prev == inf {
                continue;
            }
            // Preserve wraparound/overflow behavior narrowly: avoid overflow by saturating to INF.
            let cand = match prev.checked_add(wt) {
                Some(x) => x,
                None => inf,
            };
            if cand < dp[v] {
                dp[v] = cand;
            }
        }
    }

    // Find maximum value with weight <= capacity.
    let mut best: Option<usize> = None;
    for v in 0..=sum_values_usize {
        if dp[v] <= w_cap {
            best = Some(v);
        }
    }

    match best {
        Some(v) if v > 0 || w_cap >= 0 => {
            // If configuration exists, output maximum value.
            // Note: value 0 is always achievable with weight 0; treat as valid configuration.
            println!("{}", v);
        }
        _ => {
            // If no valid configuration exists, print an error message.
            // (This branch is practically unreachable due to value 0, but kept for fidelity.)
            println!("Error: no valid configuration");
        }
    }
}
