use std::cmp::max;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    w: i32,
    s: i32,
    v: i32,
}

fn main() {
    // Read all input (undefined behavior per spec if malformed; we simply unwrap).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    // Spec: if n is negative, behavior is undefined. We'll mirror that by not handling it.
    let n_usize = n as usize;

    let mut items = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let w: i32 = it.next().unwrap().parse().unwrap();
        let s: i32 = it.next().unwrap().parse().unwrap();
        let v: i32 = it.next().unwrap().parse().unwrap();
        items.push(Item { w, s, v });
    }

    // Sort by (s + w) ascending, matching the C comparator intent.
    // Note: C uses 32-bit int subtraction which can overflow (UB in C). Spec does not define this;
    // we use i64 to avoid overflow while preserving ordering for in-range values.
    items.sort_by_key(|x| (x.s as i64) + (x.w as i64));

    const M: usize = 20000;
    let mut dp = vec![0i64; M + 1];

    for item in items {
        // C loops j from item.s down to 0 inclusive.
        // Spec notes undefined behavior if strengths exceed 20000 or weights cause indices > 20000.
        // We intentionally do not clamp; we just avoid panics by skipping out-of-range indices,
        // which is the closest safe analogue while keeping defined behavior for valid inputs.
        if item.s < 0 {
            continue; // strength negative is outside meaningful contract; ignore safely.
        }
        let s = item.s as i64;
        let w = item.w as i64;
        let v = item.v as i64;

        let start = if s as usize > M { M } else { s as usize };
        for j in (0..=start).rev() {
            let k_i64 = (j as i64) + w;
            if k_i64 < 0 || k_i64 as usize > M {
                // Out of representable weight range (0..=20000); spec says behavior undefined.
                // We skip to keep Rust safe.
                continue;
            }
            let k = k_i64 as usize;
            dp[k] = max(dp[k], dp[j] + v);
        }
    }

    let mut ans: i64 = 0;
    for &x in &dp {
        ans = max(ans, x);
    }

    print!("{ans}\n");
}
