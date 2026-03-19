use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // No input => no defined behavior; exit quietly.
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut b: Vec<i64> = Vec::with_capacity(n);
    let mut c: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }
    for _ in 0..n {
        b.push(it.next().unwrap().parse().unwrap());
    }
    for _ in 0..n {
        c.push(it.next().unwrap().parse().unwrap());
    }

    // The specification describes a DP over subsets with O(n * 2^n) time and memory.
    // It does not define the exact cost function for transitions between states.
    // Narrowest faithful interpretation: implement the subset DP structure that
    // "minimizes a calculated cost" but, absent a defined cost, treat all transitions
    // as equal cost (0). This yields a deterministic minimal-cost solution by tie-break.
    //
    // Observable behavior: output a sequence of n integers from a[] in the order
    // determined by the computed transitions. With zero costs, we choose the
    // lexicographically smallest order by index (0..n-1) via tie-breaking.

    if n == 0 {
        // Output sequence of length 0: print nothing.
        return;
    }

    // Guard against shifting by >= number of bits in usize.
    // The spec does not constrain n; behavior for huge n is not defined.
    // We keep it simple and rely on Rust panics/overflow if n is too large.
    let total_masks: usize = 1usize << n;

    // dp[mask][last] = minimal cost to reach subset mask ending at 'last'
    // Use i128 to avoid overflow concerns in accumulation, though costs are 0 here.
    let inf: i128 = i128::MAX / 4;
    let mut dp: Vec<Vec<i128>> = vec![vec![inf; n]; total_masks];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; n]; total_masks];
    // prev[mask][last] = Some((prev_mask, prev_last))

    // Initialize singletons.
    for i in 0..n {
        let m = 1usize << i;
        dp[m][i] = 0;
        prev[m][i] = None;
    }

    // Transition: add a new element 'next' to subset.
    // Cost is unspecified; we use 0 for all transitions.
    for mask in 1usize..total_masks {
        for last in 0..n {
            if dp[mask][last] == inf {
                continue;
            }
            for next in 0..n {
                if (mask >> next) & 1 == 1 {
                    continue;
                }
                let nmask = mask | (1usize << next);

                // Unspecified cost function; treat as 0.
                // b and c are read but not used because the spec does not define how.
                let add_cost: i128 = 0;

                let cand = dp[mask][last].saturating_add(add_cost);
                if cand < dp[nmask][next] {
                    dp[nmask][next] = cand;
                    prev[nmask][next] = Some((mask, last));
                } else if cand == dp[nmask][next] {
                    // Deterministic tie-break: prefer smaller predecessor chain by choosing
                    // smaller 'last' and then smaller 'mask' (which indirectly prefers
                    // earlier indices due to iteration order).
                    if let Some((pm, pl)) = prev[nmask][next] {
                        if last < pl || (last == pl && mask < pm) {
                            prev[nmask][next] = Some((mask, last));
                        }
                    } else {
                        prev[nmask][next] = Some((mask, last));
                    }
                }
            }
        }
    }

    let full = total_masks - 1;

    // Choose best ending 'last' with minimal cost; tie-break by smallest last index.
    let mut best_last = 0usize;
    let mut best_cost = dp[full][0];
    for last in 1..n {
        let cst = dp[full][last];
        if cst < best_cost || (cst == best_cost && last < best_last) {
            best_cost = cst;
            best_last = last;
        }
    }

    // Reconstruct order by backtracking.
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut cur_mask = full;
    let mut cur_last = best_last;
    loop {
        order.push(cur_last);
        match prev[cur_mask][cur_last] {
            None => break,
            Some((pm, pl)) => {
                cur_mask = pm;
                cur_last = pl;
            }
        }
    }
    order.reverse();

    // Output the corresponding a[] values.
    // Print as space-separated integers.
    let mut out = String::new();
    for (k, &idx) in order.iter().enumerate() {
        if k > 0 {
            out.push(' ');
        }
        out.push_str(&a[idx].to_string());
    }
    print!("{out}");
}
