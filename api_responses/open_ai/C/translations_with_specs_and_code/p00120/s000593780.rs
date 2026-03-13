// translation of the described C program / specification into Rust
// Preserves semantics from the SPEC: for each input line (len followed by cake values),
// determine whether the minimal tour cost (with step cost 2*sqrt(a*b)) that
// starts at index 0, visits every item exactly once (each item must be chosen at some step),
// and finishes by returning to index 0 is < len + EPS.
// EPS = 1e-5
//
// Implementation notes:
// - Uses a DP over subsets (bitmask) with memoization.
// - The DP state and transitions follow the specification: the final step is visiting index 0,
//   and the starting position is index 0 (not marked visited initially).
// - Input parsing reads input line-by-line; each non-empty line is treated as one case.
// - Assumes 1 <= n <= 12 as per SPEC. If a line has no cake values, it is skipped.
// - Uses safe Rust only.

use std::io::{self, BufRead};
use std::f64;

const EPS: f64 = 1e-5;
const MAX_N: usize = 12;

fn cost(a: f64, b: f64) -> f64 {
    // step cost = 2 * sqrt(a * b)
    // Follow SPEC: cake values are assumed non-negative; if negative, sqrt yields NaN as in C.
    2.0 * (a * b).sqrt()
}

fn main() {
    let stdin = io::stdin();
    let mut out_lines: Vec<String> = Vec::new();

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(s) => s,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            // skip blank lines (C behavior with scanf would differ on malformed input;
            // SPEC assumes each case is on one line so skipping blank lines is reasonable)
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        // parse len
        let len_tok = match parts.next() {
            Some(t) => t,
            None => continue,
        };
        let len: f64 = match len_tok.parse() {
            Ok(v) => v,
            Err(_) => continue, // malformed: skip line
        };

        // parse cake values
        let mut cakes: Vec<f64> = Vec::new();
        for tok in parts {
            if cakes.len() >= MAX_N {
                // SPEC: behavior for n > 12 is undefined. We stop parsing further values
                // (practical choice to avoid unbounded memory). This does not contradict the SPEC.
                break;
            }
            match tok.parse::<f64>() {
                Ok(v) => cakes.push(v),
                Err(_) => {
                    // malformed token; treat as skip of this case
                    cakes.clear();
                    break;
                }
            }
        }

        if cakes.is_empty() {
            // No cake values parsed -> per SPEC at least one cake value is expected.
            // We skip this malformed line.
            continue;
        }

        let n = cakes.len();
        // prepare DP table: (1<<n) * n entries
        let full_mask = (1usize << n) - 1;
        let dp_size = (1usize << n) * n;
        let mut dp: Vec<Option<f64>> = vec![None; dp_size];

        // recursive closure for tsp with memoization
        // state: mask = visited items mask (bits set mean that item has been chosen/visited),
        //        now = current position index
        // note: according to SPEC/C program, starting call is (mask=0, now=0)
        // and the final successful base state is mask == full_mask && now == 0 returning 0.
        // We'll implement equivalent formulation where base case returns cost(now, 0) when mask == full_mask,
        // and initial call uses mask = 0 and now = 0. This matches the C behaviour for total cost.
        fn tsp(
            mask: usize,
            now: usize,
            n: usize,
            full_mask: usize,
            cakes: &[f64],
            dp: &mut [Option<f64>],
        ) -> f64 {
            let idx = mask * n + now;
            if let Some(v) = dp[idx] {
                return v;
            }

            // base: all visited -> no more selections; if we're at now==0 this returns 0,
            // otherwise must return cost to select 0 as last step. To match C behavior,
            // when mask==full_mask and now==0 the cost should be 0; when mask==full_mask and now!=0
            // there is no further selection (but in our transition scheme we always select until mask full,
            // and we used returning cost approach below). For consistency with the original algorithm,
            // return cost(now, 0) when mask == full_mask.
            if mask == full_mask {
                let res = cost(cakes[now], cakes[0]);
                dp[idx] = Some(res);
                return res;
            }

            let mut best = f64::INFINITY;
            for next in 0..n {
                if (mask >> next) & 1 == 0 {
                    let next_mask = mask | (1usize << next);
                    let sub = tsp(next_mask, next, n, full_mask, cakes, dp);
                    // step cost from now -> next
                    let step = cost(cakes[now], cakes[next]);
                    let cand = sub + step;
                    if cand < best {
                        best = cand;
                    }
                }
            }

            dp[idx] = Some(best);
            best
        }

        // compute minimal tour starting at now=0 with no items marked visited (mask=0)
        let minimal = tsp(0, 0, n, full_mask, &cakes, &mut dp);

        if minimal < len + EPS {
            out_lines.push("OK".to_string());
        } else {
            out_lines.push("NA".to_string());
        }
    }

    // print outputs exactly one per processed case, each followed by newline
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    use std::io::Write;
    for line in out_lines {
        let _ = writeln!(handle, "{}", line);
    }
}
