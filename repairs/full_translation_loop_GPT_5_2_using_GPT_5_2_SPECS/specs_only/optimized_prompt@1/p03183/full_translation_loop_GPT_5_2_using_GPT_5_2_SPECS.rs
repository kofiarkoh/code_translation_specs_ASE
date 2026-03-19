use std::cmp::max;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    w: usize,
    s: usize,
    v: i64,
}

fn main() {
    // Read all input; malformed/incomplete input => behavior not defined by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // Narrow interpretation: no input => no output required by spec.
    };

    let mut items: Vec<Item> = Vec::with_capacity(n);
    for _ in 0..n {
        let w: usize = it.next().unwrap().parse().unwrap();
        let s: usize = it.next().unwrap().parse().unwrap();
        let v: i64 = it.next().unwrap().parse().unwrap();
        items.push(Item { w, s, v });
    }

    // Sort by (w + s) ascending; common and consistent with the capacity bound.
    // If w+s > 20000, spec says behavior is not defined; we do not guard.
    items.sort_by_key(|x| x.w + x.s);

    const CAP: usize = 20000;
    const NEG_INF: i64 = i64::MIN / 4;

    // dp[tw] = maximum total value achievable with total stack weight exactly tw.
    // Initialize unreachable states to NEG_INF; dp[0] = 0.
    let mut dp = vec![NEG_INF; CAP + 1];
    dp[0] = 0;

    for item in items {
        // For an item with strength s, it can be placed under any stack whose total weight
        // (above it) is <= s. New total weight becomes tw + w.
        // Iterate tw descending to ensure each item used at most once.
        let max_tw = item.s.min(CAP);
        for tw in (0..=max_tw).rev() {
            let cur = dp[tw];
            if cur == NEG_INF {
                continue;
            }
            let new_w = tw + item.w;
            // Spec implies potential out-of-bounds if w+s > 20000; we keep within dp bounds.
            if new_w <= CAP {
                dp[new_w] = max(dp[new_w], cur + item.v);
            }
        }
    }

    let ans = dp.into_iter().max().unwrap_or(0);
    println!("{}", ans);
}
