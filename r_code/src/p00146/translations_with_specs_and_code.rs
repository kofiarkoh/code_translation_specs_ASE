// translate.rs
// Translation of the given C program to Rust following the provided specification.
// Note: The original C code contained a precedence bug in the condition
//   if((1<<i)&j==0)continue;
// which is parsed as (1<<i) & (j==0) in C due to operator precedence,
// producing incorrect behavior. The specification describes the intended
// behavior (checking whether bit i is set in mask j). This Rust port
// implements the intended behavior from the specification.
//
// The program reads:
//   n
//   a[0] b[0] c[0]
//   ...
//   a[n-1] b[n-1] c[n-1]
// and prints the labels a[...] in the order that minimizes the total cost,
// as defined in the specification.

use std::io::{self, Read};

fn main() {
    // Read all input tokens as integers
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");
    let mut it = s.split_whitespace();

    // Parse n
    let n: usize = match it.next() {
        Some(tok) => tok.parse().expect("parse n"),
        None => return, // malformed input; spec says behavior undefined; just exit
    };

    // Read arrays a, b, c
    let mut a: Vec<i32> = Vec::with_capacity(n);
    let mut b: Vec<i32> = Vec::with_capacity(n);
    let mut c: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        // Expect three ints per item
        let ai: i32 = it.next().expect("missing a").parse().expect("parse a");
        let bi: i32 = it.next().expect("missing b").parse().expect("parse b");
        let ci: i32 = it.next().expect("missing c").parse().expect("parse c");
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    // Practical precondition: n <= 16 (spec). We don't enforce, but behavior may be undefined for larger n.
    if n == 0 {
        // Spec expects n >= 1; undefined otherwise. We'll exit silently.
        return;
    }

    let full_mask: usize = (1usize << n) - 1;
    let masks = 1usize << n;

    // dp[k][mask] = minimal cost to reach state where visited set == mask and last visited is k
    // Use a flat Vec of length n * masks for cache friendliness
    let inf = 1e18_f64;
    let mut d = vec![vec![inf; masks]; n];
    // fr[k][mask] = previous node index i (or -1)
    let mut fr = vec![vec![-1i32; masks]; n];

    // initialize starting positions: visiting single node i with cost 0
    for i in 0..n {
        let mask = 1usize << i;
        d[i][mask] = 0.0;
    }

    // cost function f(i, mask_j, k)
    let f = |i: usize, mask_j: usize, k: usize, b: &Vec<i32>, c: &Vec<i32>, n: usize| -> f64 {
        if mask_j == 0 {
            return 0.0;
        }
        // w = sum_{l in S} c[l]*20
        let mut w: i64 = 0;
        for l in 0..n {
            if (mask_j >> l) & 1 != 0 {
                w += (c[l] as i64) * 20;
            }
        }
        let coeff = (w as f64 + 70.0) / 2000.0;
        let diff = (b[k] - b[i]).abs() as f64;
        diff * coeff
    };

    // DP over masks
    for mask in 0..masks {
        for i in 0..n {
            // Only consider states where i is in mask
            if ((mask >> i) & 1) == 0 {
                continue;
            }
            let cur_cost = d[i][mask];
            if cur_cost >= inf {
                continue;
            }
            for k in 0..n {
                if ((mask >> k) & 1) != 0 {
                    continue; // k already visited
                }
                let new_mask = mask | (1usize << k);
                let cost = cur_cost + f(i, mask, k, &b, &c, n);
                // Match C's tie-breaking: update only if strictly less
                if cost < d[k][new_mask] {
                    d[k][new_mask] = cost;
                    fr[k][new_mask] = i as i32;
                }
            }
        }
    }

    // Find end node with minimal cost at full mask
    let mut best_last = 0usize;
    for i in 0..n {
        if d[i][full_mask] < d[best_last][full_mask] {
            best_last = i;
        }
    }

    // Reconstruct path r[0..n-1]
    let mut r = vec![0usize; n];
    r[n - 1] = best_last;
    let mut mask = full_mask;
    for idx in 1..n {
        let last = r[n - idx];
        let prev = fr[last][mask];
        // prev is expected to be >= 0
        let prev_usize = if prev >= 0 {
            prev as usize
        } else {
            // Should not happen for well-formed inputs per spec; fallback to 0
            0usize
        };
        r[n - idx - 1] = prev_usize;
        // remove last from mask
        mask &= !(1usize << last);
    }

    // Print labels a[r[0]] ... a[r[n-1]] separated by spaces and newline
    let mut out = String::new();
    for i in 0..n {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{}", a[r[i]]));
    }
    out.push('\n');
    print!("{}", out);
}
