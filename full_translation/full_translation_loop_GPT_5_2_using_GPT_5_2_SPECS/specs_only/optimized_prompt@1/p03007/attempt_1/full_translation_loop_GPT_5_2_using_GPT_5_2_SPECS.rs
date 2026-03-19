use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // Sort ascending for deterministic construction
    a.sort_unstable();

    let has_neg = a.first().unwrap() < &0;
    let has_nonneg = a.last().unwrap() >= &0;

    let mut ops: Vec<(i64, i64)> = Vec::with_capacity(n.saturating_sub(1));
    let ans: i64;

    if has_neg && has_nonneg {
        // Mixed signs: maximum is sum |ai|
        // Construction:
        // - Start with the smallest (most negative) as accumulator.
        // - Subtract all non-negative values except the largest from it (making it more negative).
        // - Then take the largest non-negative as accumulator and subtract all remaining negatives (including the built accumulator).
        let mut neg: Vec<i64> = a.iter().cloned().filter(|&x| x < 0).collect();
        let mut nonneg: Vec<i64> = a.iter().cloned().filter(|&x| x >= 0).collect();

        // nonneg is non-empty, neg is non-empty
        let mut acc = neg[0];
        // subtract all nonneg except the largest
        for &x in &nonneg[..nonneg.len() - 1] {
            ops.push((acc, x));
            acc = acc - x;
        }
        // now subtract all remaining negatives from the largest nonneg
        let mut acc2 = nonneg[nonneg.len() - 1];
        for &x in &neg[1..] {
            ops.push((acc2, x));
            acc2 = acc2 - x;
        }
        // include the built negative accumulator
        ops.push((acc2, acc));
        acc2 = acc2 - acc;

        ans = acc2;
    } else if has_nonneg {
        // All non-negative: maximum is sum|ai| - 2*min(ai) = sum(ai) - 2*min
        // Construction:
        // - Let m = smallest.
        // - Build a negative accumulator by subtracting all middle elements from m.
        // - Then subtract that accumulator from the largest.
        // This uses each original value exactly once as an operand.
        let m = a[0];
        let maxv = a[n - 1];

        let mut acc = m;
        for &x in &a[1..n - 1] {
            ops.push((acc, x));
            acc = acc - x;
        }
        ops.push((maxv, acc));
        ans = maxv - acc;
    } else {
        // All non-positive: maximum is sum|ai| - 2*|max(ai)| = sum|ai| + 2*max(ai)
        // Construction (symmetric to all-nonnegative case):
        // - Let M = largest (closest to zero, still <= 0).
        // - Build a positive accumulator by subtracting all other elements (more negative) from M.
        // - Then subtract that accumulator from the smallest (most negative).
        let maxv = a[n - 1];
        let minv = a[0];

        let mut acc = maxv;
        for &x in &a[1..n - 1] {
            ops.push((acc, x));
            acc = acc - x;
        }
        ops.push((minv, acc));
        ans = minv - acc;
    }

    // Output
    // Exactly n-1 operations must be emitted.
    // If the construction is correct, ops.len() == n-1.
    let mut out = String::new();
    out.push_str(&format!("{}\n", ans));
    for (x, y) in ops {
        out.push_str(&format!("{} {}\n", x, y));
    }
    print!("{}", out);
}
