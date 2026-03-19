use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(x) = it.next() {
            a.push(x.parse::<i64>().unwrap());
        } else {
            return;
        }
    }

    // Absolute sum (AtCoder p03007 style): sum of absolute values.
    // Use i128 for intermediate to avoid overflow; final output fits typical constraints.
    let mut sum_abs: i128 = 0;
    for &v in &a {
        sum_abs += (v as i128).abs();
    }

    // This task corresponds to constructing N-1 operations (x y) that combine numbers
    // to achieve the maximum possible final value, which is sum_abs, and printing the
    // operations in a specific order.
    //
    // Standard construction:
    // - If all numbers are non-negative: use the smallest as the "negative accumulator"
    //   by subtracting all other positives from it, then subtract that result from the
    //   largest.
    // - If all numbers are non-positive: use the largest (closest to zero) as the
    //   "positive accumulator" by subtracting other negatives from it.
    // - Mixed: use one negative accumulator and one positive accumulator, first absorb
    //   positives into the negative accumulator, then absorb remaining negatives into
    //   the positive accumulator, finally combine.
    a.sort();

    let mut ops: Vec<(i64, i64)> = Vec::with_capacity(n.saturating_sub(1));

    let all_nonneg = a[0] >= 0;
    let all_nonpos = a[n - 1] <= 0;

    if all_nonneg {
        // a[0] is smallest, a[n-1] is largest.
        let mut x = a[0];
        for i in 1..n - 1 {
            ops.push((x, a[i]));
            x = x - a[i];
        }
        // Final combine with largest.
        ops.push((a[n - 1], x));
    } else if all_nonpos {
        // All <= 0. Use the largest (closest to zero) as accumulator.
        let mut x = a[n - 1];
        for i in 0..n - 1 {
            ops.push((x, a[i]));
            x = x - a[i];
        }
    } else {
        // Mixed.
        // Split into negatives (including 0? treat 0 as non-negative) and positives.
        // We need one negative accumulator and one positive accumulator.
        let mut neg: Vec<i64> = Vec::new();
        let mut pos: Vec<i64> = Vec::new();
        for &v in &a {
            if v < 0 {
                neg.push(v);
            } else {
                pos.push(v);
            }
        }

        // Negative accumulator starts from the most negative.
        let mut x = neg[0];
        // Positive accumulator starts from the largest positive (or zero if no positive, but mixed ensures some >0).
        let mut y = *pos.last().unwrap();

        // Absorb all other positives (except y) into x: x = x - p
        for &p in &pos[..pos.len() - 1] {
            ops.push((x, p));
            x = x - p;
        }

        // Absorb remaining negatives (except x's original neg[0]) into y: y = y - n
        for &m in &neg[1..] {
            ops.push((y, m));
            y = y - m;
        }

        // Final combine: y - x
        ops.push((y, x));
    }

    let mut out = String::new();
    out.push_str(&format!("{}\n", sum_abs));
    for (x, y) in ops {
        out.push_str(&format!("{} {}\n", x, y));
    }
    print!("{out}");
}
