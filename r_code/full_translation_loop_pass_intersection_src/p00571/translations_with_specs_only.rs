use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Item {
    x: i64,
    w: i64,
}

fn main() {
    // Read entire stdin; no explicit error handling per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // Unspecified if missing; exit quietly.
    };

    // Capacity precondition: at most 500,001 items. If violated, behavior unspecified.
    let mut items: Vec<Item> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        items.push(Item { x, w });
    }

    // Sort by non-decreasing x, then non-decreasing w.
    items.sort_unstable_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Equal => a.w.cmp(&b.w),
        other => other,
    });

    // We need:
    // max_{l<=r} (sum_{k=l..r} w_k - (x_r - x_l))
    // Let prefix S[i] = sum_{k=1..i} w_k, with S[0]=0.
    // Expression for fixed r:
    // (S[r] - S[l-1]) - (x_r - x_l) = (S[r] - x_r) - (S[l-1] - x_l)
    // So for each r, maximize by minimizing (S[l-1] - x_l) over l in [1..r].
    //
    // Note: l uses x_l (not x_{l-1}), so we track min over l of (S[l-1] - x_l).
    // This can be updated as we advance r.

    let mut prefix: i64 = 0;

    // Initialize with l = 1 candidate before processing r=1:
    // min_val = S[0] - x_1 = -x_1
    // If n==0, spec doesn't define; we do nothing.
    if n == 0 {
        return;
    }

    let mut min_val: i64 = 0 - items[0].x;

    // Maximum over non-empty subsequences; at least 0 since w >= 0.
    let mut best: i64 = 0;

    for (idx, item) in items.iter().enumerate() {
        // Update prefix to S[r]
        prefix += item.w;

        // Candidate best ending at r = idx+1:
        // (S[r] - x_r) - min_{l<=r}(S[l-1] - x_l)
        let cand = (prefix - item.x) - min_val;
        if cand > best {
            best = cand;
        }

        // Prepare min_val for future r: consider l = (idx+2) using S[r] - x_{r+1}
        // That requires x_{r+1}, so update only if there is a next item.
        if idx + 1 < n {
            let next_x = items[idx + 1].x;
            let v = prefix - next_x; // S[r] - x_{r+1}
            if v < min_val {
                min_val = v;
            }
        }
    }

    println!("{}", best);
}
