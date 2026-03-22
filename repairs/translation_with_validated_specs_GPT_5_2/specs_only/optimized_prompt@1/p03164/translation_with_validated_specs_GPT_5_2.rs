use std::io::{self, Read};
use std::process;

const MAX_ITEMS: usize = 128;
const MAX_V: usize = 112_344;
const TABLE_LEN: usize = MAX_V + 1;

fn main() {
    // Read all input; on any read/parse failure, exit nonzero with no guaranteed output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let w_limit: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Preconditions say n <= 128 and values within range; we do not add extra behavior beyond spec.
    // Still, we must not panic; if n exceeds supported maximum, we only process what we can read
    // but keep semantics narrow: the spec assumes it won't happen.
    let n = n.min(MAX_ITEMS);

    let mut items: Vec<(i64, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        let w: i64 = match it.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let v: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        // Spec preconditions guarantee 0 <= v <= MAX_V.
        items.push((w, v));
    }

    // DP invariant: dp[val] = minimum achievable weight to get total value exactly val,
    // or sentinel meaning "not achievable within tracked bounds".
    // Value 0 is always achievable with weight 0.
    let inf: i64 = i64::MAX / 4;
    let mut dp = vec![inf; TABLE_LEN];
    dp[0] = 0;

    for (w, v) in items {
        // 0/1 knapsack by value: iterate values descending to avoid reusing the same item.
        if v > MAX_V {
            // Precondition says this won't happen; ignore to avoid out-of-bounds.
            continue;
        }
        for val in (v..=MAX_V).rev() {
            let prev = dp[val - v];
            if prev == inf {
                continue;
            }
            // Preconditions state intermediate accumulated weights fit in integer range.
            let cand = prev + w;
            if cand < dp[val] {
                dp[val] = cand;
            }
        }
    }

    // Select the largest value with achievable weight <= limit.
    let mut best: Option<usize> = None;
    for val in (0..=MAX_V).rev() {
        if dp[val] <= w_limit {
            best = Some(val);
            break;
        }
    }

    match best {
        Some(v) => {
            // Postcondition: printed value within [0, 112344].
            println!("{v}");
        }
        None => {
            // Specific internal failure case: no achievable value within bounds.
            // Spec requires a diagnostic message and successful termination.
            println!("error");
        }
    }
}
