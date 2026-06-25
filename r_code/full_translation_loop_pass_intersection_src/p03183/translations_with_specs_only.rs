use std::io::{self, Read};

const LIM: usize = 20_000;

#[derive(Clone, Copy)]
struct Item {
    w: usize,
    s: usize,
    v: i32,
}

fn main() {
    // Read entire stdin; undefined behavior on malformed input is acceptable per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    // If n is negative, behavior is undefined by contract; we do not guard.
    let n_usize = n as usize;

    let mut items: Vec<Item> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let w_i: i32 = it.next().unwrap().parse().unwrap();
        let s_i: i32 = it.next().unwrap().parse().unwrap();
        let v_i: i32 = it.next().unwrap().parse().unwrap();

        // Strength is assumed non-negative; otherwise behavior not meaningfully defined.
        // We cast directly; negative values will wrap to large usize, matching "undefined".
        items.push(Item {
            w: w_i as usize,
            s: s_i as usize,
            v: v_i,
        });
    }

    // Standard approach: sort by (w + s) ascending to allow DP by total weight.
    // This preserves the existence of some feasible ordering for any chosen subset.
    items.sort_by_key(|x| x.w + x.s);

    // dp[tw] = maximum value achievable with total stack weight exactly tw,
    // considering only stacks with total weight within 0..=LIM.
    // Unreachable states are represented by a large negative sentinel.
    let neg_inf: i32 = i32::MIN / 4;
    let mut dp = vec![neg_inf; LIM + 1];
    dp[0] = 0;

    for item in items {
        // Per spec, if item.s > 20000 or weights exceed 20000, original program may go OOB.
        // We intentionally do not add extra guards beyond the DP table bounds.
        // We only iterate within 0..=LIM, so if item.w > LIM, no transitions occur.
        let w = item.w;
        let s = item.s;
        let v = item.v;

        // To place this item below an existing stack of weight `above`,
        // we need `above <= s`, and new total weight is `above + w`.
        // Iterate `above` descending to ensure each item used at most once.
        let max_above = if s > LIM { LIM } else { s };
        for above in (0..=max_above).rev() {
            let cur = dp[above];
            if cur == neg_inf {
                continue;
            }
            let new_w = above + w;
            if new_w <= LIM {
                let cand = cur.saturating_add(v);
                if cand > dp[new_w] {
                    dp[new_w] = cand;
                }
            }
        }
    }

    let mut ans = 0i32;
    for &x in &dp {
        if x > ans {
            ans = x;
        }
    }

    print!("{}", ans);
}
