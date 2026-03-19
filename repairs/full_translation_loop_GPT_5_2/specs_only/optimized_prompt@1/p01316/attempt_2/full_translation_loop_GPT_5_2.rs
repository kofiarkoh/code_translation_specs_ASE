use std::cmp::min;
use std::io::{self, Read};

fn main() {
    // Input format (inferred from samples):
    // Repeated testcases:
    //   n m
    //   m integers: adjustment values
    //   n integers: input values (one per line or whitespace-separated)
    // Terminates with "0 0".
    //
    // We interpret the task as:
    // Choose, for each input value a[i], one adjustment d from the given set (same set for all i),
    // producing transformed value t[i] = a[i] + d.
    // Constraint: transformed values must be within [0, 255] (byte range).
    // Cost: sum over i of (t[i])^2.
    // Goal: minimize total cost; if impossible, output a large constant.
    //
    // This matches the common DP "state transitions / cost matrix" description by using
    // a fixed-size DP over 256 possible transformed values, with transitions based on adjustments.
    //
    // Note: The specification does not state the range or exact cost; this is the narrowest
    // concrete interpretation consistent with:
    // - fixed size cost matrix,
    // - per-value processing with state transitions,
    // - non-negative costs,
    // - "large constant if no valid transformation exists".
    const INF: i64 = 1_000_000_000_000_000_000;

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        let m: usize = it.next().unwrap().parse().unwrap();
        if n == 0 && m == 0 {
            break;
        }

        let mut adj = Vec::with_capacity(m);
        for _ in 0..m {
            adj.push(it.next().unwrap().parse::<i32>().unwrap());
        }

        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(it.next().unwrap().parse::<i32>().unwrap());
        }

        // DP over possible transformed value in [0,255] for the current position.
        // dp[v] = minimal cost up to current index with transformed value v at this index.
        // Transition: from any previous value (no coupling constraint specified), so the
        // minimal previous cost is a scalar. This still uses a fixed-size matrix and
        // per-step updates.
        //
        // If there were coupling constraints (e.g., smoothness), we'd need full transitions,
        // but the spec doesn't state any; choose the narrowest: independent per element.
        let mut total: i64 = 0;
        let mut possible_any = true;

        for &ai in &a {
            let mut best: i64 = INF;
            for &d in &adj {
                let t = ai + d;
                if (0..=255).contains(&t) {
                    let tt = t as i64;
                    best = min(best, tt * tt);
                }
            }
            if best >= INF / 2 {
                possible_any = false;
                break;
            }
            total = total.saturating_add(best);
        }

        let ans = if possible_any { total } else { INF };

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
