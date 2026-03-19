use std::cmp::max;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Node {
    idx: usize,
    val: i64,
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Narrowest reasonable interpretation: if any of n/m/k missing, do nothing.
    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m_in: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let k: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Preconditions say non-negative; we clamp negative m to 0 to avoid inventing behavior.
    let m: usize = if m_in < 0 { 0 } else { m_in as usize };

    let mut a = vec![0i64; n];
    for i in 0..n {
        if let Some(x) = it.next() {
            a[i] = x.parse().unwrap();
        } else {
            // Specification requires exactly n integers; if not present, do nothing.
            return;
        }
    }

    // The specification describes:
    // - DP array updated over k iterations
    // - a "node structure" maintaining indices/values
    // - linear time per iteration
    //
    // The narrowest faithful implementation consistent with these constraints is:
    // For each iteration t=1..k, compute:
    //   dp_new[i] = max(dp_old[i], a[i] * t + max_{j <= i-m} dp_old[j])
    // where the gap constraint is enforced by only allowing transitions from indices at least m behind.
    //
    // This uses a monotone "best so far" node (index/value) updated as i advances, O(n) per iteration.
    //
    // Corner cases:
    // - If i < m, no transition is possible; dp_new[i] = dp_old[i].
    // - dp values are non-decreasing across iterations due to max with dp_old[i] (invariant).
    // - If k == 0, answer is 0 (dp initialized to 0).
    let mut dp = vec![0i64; n];

    for t in 1..=k {
        let mut dp_new = dp.clone();

        // Maintain best dp[j] among eligible j as i increases.
        // We update eligibility when i >= m by incorporating j = i - m.
        let mut have_best = false;
        let mut best = Node { idx: 0, val: 0 };

        for i in 0..n {
            if i >= m {
                let j = i - m;
                let cand = dp[j];
                if !have_best || cand > best.val {
                    best = Node { idx: j, val: cand };
                    have_best = true;
                }
            }

            if have_best {
                // Weighted contribution: a[i] * t.
                // Use i64 arithmetic; overflow behavior is not specified, so we keep Rust's
                // defined two's-complement wrapping only in release? No: Rust panics in debug.
                // To avoid debug/release divergence, use wrapping ops explicitly.
                let w = a[i].wrapping_mul(t as i64);
                let trans = best.val.wrapping_add(w);
                dp_new[i] = max(dp_new[i], trans);
            }
        }

        dp = dp_new;
    }

    let ans = dp.into_iter().max().unwrap_or(0);
    print!("{ans}");
}
