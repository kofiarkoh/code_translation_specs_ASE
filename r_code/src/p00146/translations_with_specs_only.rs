use std::io::{self, Read};

fn main() {
    // Read all input tokens as integers (i64).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut iter = input.split_whitespace();
    // Parse n
    let n: usize = match iter.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v as usize,
        None => return,
    };

    // Read arrays a, b, c as i64
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    let mut c = Vec::with_capacity(n);
    for _ in 0..n {
        // Per specification, input is well-formed; we simply assume tokens exist.
        let ai = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let bi = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let ci = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    // For n == 0 the behavior is undefined per spec; handle minimally by printing nothing.
    if n == 0 {
        return;
    }

    // Precompute sum_c for each mask
    let masks = 1usize << n;
    let mut sum_c = vec![0i64; masks];
    for mask in 1..masks {
        // get lowest set bit
        let lsb = mask & (!mask + 1);
        let bit = lsb.trailing_zeros() as usize;
        let prev = mask ^ lsb;
        sum_c[mask] = sum_c[prev] + c[bit];
    }

    // DP arrays: dp[mask][last] = minimal cost to visit 'mask' with 'last' visited last.
    // pred[mask][last] = previous 'last' index (or -1 if start)
    let inf = std::f64::INFINITY;
    let mut dp = vec![vec![inf; n]; masks];
    let mut pred = vec![vec![-1i16; n]; masks];

    // Initialization: single-element masks have cost 0 and no predecessor
    for i in 0..n {
        let mask = 1usize << i;
        dp[mask][i] = 0.0;
        pred[mask][i] = -1;
    }

    // Epsilon for floating comparison to preserve deterministic tie-breaking:
    // we only replace when new_cost < current_cost - eps
    let eps = 1e-12f64;

    // DP transitions
    for mask in 1..masks {
        for last in 0..n {
            if (mask & (1usize << last)) == 0 {
                continue;
            }
            let curr_cost = dp[mask][last];
            if !curr_cost.is_finite() {
                continue;
            }
            // Try to extend by adding k not in mask
            for k in 0..n {
                if (mask & (1usize << k)) != 0 {
                    continue;
                }
                let next_mask = mask | (1usize << k);
                // incremental cost: if S (mask) is empty -> 0; but mask is non-empty here
                // S includes 'last' as per specification
                let dist = (b[k] - b[last]).abs() as f64;
                let factor = (70.0 + 20.0 * (sum_c[mask] as f64)) / 2000.0;
                let inc = dist * factor;
                let new_cost = curr_cost + inc;
                let prev_cost = dp[next_mask][k];
                if new_cost < prev_cost - eps {
                    dp[next_mask][k] = new_cost;
                    pred[next_mask][k] = last as i16;
                }
                // If costs are effectively equal, do not update to preserve first-found deterministic tie-breaking.
            }
        }
    }

    // Reconstruct best endpoint
    let full = masks - 1;
    let mut best_last: isize = -1;
    let mut best_cost = inf;
    for last in 0..n {
        let cst = dp[full][last];
        if cst.is_finite() && cst < best_cost - eps {
            best_cost = cst;
            best_last = last as isize;
        }
        // If tie within eps, keep earlier (first found) to be deterministic.
    }

    // If no path found (shouldn't happen for valid inputs), exit quietly.
    if best_last < 0 {
        return;
    }

    // Reconstruct sequence in reverse
    let mut order_rev = Vec::with_capacity(n);
    let mut mask = full;
    let mut last = best_last as i16;
    while last >= 0 {
        order_rev.push(last as usize);
        let p = pred[mask][last as usize];
        mask ^= 1usize << (last as usize);
        last = p;
    }
    // At this point, mask should be 0 and we've collected all visited in reverse order.
    order_rev.reverse();

    // Print labels a[r[0]] ... a[r[n-1]] separated by spaces and newline
    let mut out = String::new();
    for (i, &idx) in order_rev.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{}", a[idx]));
    }
    out.push('\n');
    print!("{}", out);
}
