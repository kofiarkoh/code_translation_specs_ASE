use std::f64;
use std::io::{self, BufRead};

fn main() {
    // EPS as specified
    const EPS: f64 = 1e-5;

    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break, // I/O error: stop processing (narrow reasonable behavior)
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Tokenize the line: first token is len, the rest are cake values
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 2 {
            // Specification assumes at least one cake value per line.
            // Here we choose to skip malformed lines.
            continue;
        }

        // Parse len
        let len: f64 = match toks[0].parse() {
            Ok(v) => v,
            Err(_) => continue, // malformed len token: skip line
        };

        // Parse cake values
        let mut cake: Vec<f64> = Vec::with_capacity(toks.len() - 1);
        let mut parse_error = false;
        for &t in toks.iter().skip(1) {
            match t.parse::<f64>() {
                Ok(v) => cake.push(v),
                Err(_) => {
                    parse_error = true;
                    break;
                }
            }
        }
        if parse_error || cake.is_empty() {
            continue; // skip malformed line
        }

        let n = cake.len();
        // Specification restricts n to 1..=12. If larger, behavior unspecified.
        if n > 12 {
            // Narrow reasonable behavior: treat as malformed and skip.
            continue;
        }

        // Precompute distances: cost between i and j is 2 * sqrt(cake[i] * cake[j])
        let mut dist = vec![vec![0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                // Using the direct formula 2.0 * sqrt(a*b). If cake values are negative,
                // this can produce NaN, which matches the unspecified behavior in the spec.
                dist[i][j] = 2.0 * (cake[i] * cake[j]).sqrt();
            }
        }

        let full_mask = (1usize << n) - 1;

        // Special-case n == 1: starting at 0 and having visited all items implies no moves,
        // so minimal tour cost is 0.0 (this avoids counting a self-loop distance[0][0]).
        let minimal = if n == 1 {
            0.0f64
        } else {
            // DP over subsets: dp[mask][last] = minimal cost to reach `last` having visited `mask`.
            // We'll store as a flat vector of length (1<<n) * n, indexed by mask * n + last.
            let size = (1usize << n) * n;
            let mut dp = vec![f64::INFINITY; size];

            let start_mask = 1usize << 0;
            dp[start_mask * n + 0] = 0.0;

            // Iterate over all masks that include the start bit (bit 0).
            // For correctness we iterate all masks but skip those that don't include start.
            for mask in 0usize..=(1usize << n) - 1 {
                if (mask & start_mask) == 0 {
                    continue;
                }
                for last in 0..n {
                    let idx = mask * n + last;
                    let cur = dp[idx];
                    if !cur.is_finite() {
                        continue;
                    }
                    // Try to go to any unvisited node `next`
                    for next in 0..n {
                        if (mask & (1usize << next)) != 0 {
                            continue;
                        }
                        let next_mask = mask | (1usize << next);
                        let next_idx = next_mask * n + next;
                        let cand = cur + dist[last][next];
                        if cand < dp[next_idx] {
                            dp[next_idx] = cand;
                        }
                    }
                }
            }

            // Close the tour by returning to 0 from any last.
            let mut best = f64::INFINITY;
            for last in 0..n {
                let val = dp[full_mask * n + last];
                if !val.is_finite() {
                    continue;
                }
                let total = val + dist[last][0];
                if total < best {
                    best = total;
                }
            }
            best
        };

        // Compare as specified: print "OK" if minimal < len + EPS, otherwise "NA".
        if minimal < len + EPS {
            println!("OK");
        } else {
            println!("NA");
        }
    }
}
