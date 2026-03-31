use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let x: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions say N is non-negative and <= 200000; behavior unspecified otherwise.
    // We keep computations in i64 as specified (no overflow assumed by preconditions).
    if x == 0 && d == 0 {
        println!("1");
        return;
    }
    if d == 0 {
        // Possible sums are k*X for k=0..N; distinct count is N+1 (even if X=0 handled above).
        println!("{}", n + 1);
        return;
    }
    if x == 0 {
        // Distinct sums are D * t where t ranges over all subset sums of {0..N-1},
        // which yields all integers in [0, N(N-1)/2].
        let nn = n;
        let ans = nn * (nn - 1) / 2 + 1;
        println!("{}", ans);
        return;
    }

    // General case:
    // Any subset S of size m has sum:
    //   sum_{k in S} (X + kD) = mX + D * (sum_{k in S} k)
    // Let D be possibly negative; normalize by flipping sign so D > 0.
    // This preserves distinctness because it is a bijection on sums.
    let mut x = x;
    let mut d = d;
    if d < 0 {
        d = -d;
        x = -x;
    }

    // For fixed m, sum_{k in S} k ranges over an interval [min_t(m), max_t(m)]
    // with all integers achievable (for k=0..N-1). Specifically:
    //   min_t(m) = 0+1+...+(m-1) = m(m-1)/2
    //   max_t(m) = (N-m)+...+(N-1) = m(2N-m-1)/2
    // Then sums for fixed m are:
    //   mX + D * t, for t in [min_t(m), max_t(m)].
    //
    // Distinctness across different m depends on residues modulo D:
    //   mX + D*t ≡ mX (mod D).
    // Group by r = (mX mod D) in [0, D-1], and within each group merge intervals
    // of the corresponding quotient q = (mX - r)/D + t.
    //
    // Total distinct sums = sum over residues of total covered integer points in merged intervals.
    let mut groups: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

    let n_i64 = n;
    for m in 0..=n_i64 {
        // Compute r = (m*x) mod d in [0, d-1] using Euclidean remainder.
        let mx = m * x;
        let r = mx.rem_euclid(d);
        let base = (mx - r) / d;

        let min_t = m * (m - 1) / 2;
        let max_t = m * (2 * n_i64 - m - 1) / 2;

        let l = base + min_t;
        let rr = base + max_t;

        groups.entry(r).or_default().push((l, rr));
    }

    let mut ans: i128 = 0;
    for (_r, mut intervals) in groups {
        intervals.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut cur_l: i64 = 0;
        let mut cur_r: i64 = -1;
        let mut first = true;

        for (l, r) in intervals {
            if first {
                cur_l = l;
                cur_r = r;
                first = false;
                continue;
            }
            if l > cur_r + 1 {
                // Disjoint; finalize previous.
                ans += (cur_r as i128 - cur_l as i128 + 1);
                cur_l = l;
                cur_r = r;
            } else {
                // Overlapping or adjacent; merge.
                cur_l = min(cur_l, l);
                cur_r = max(cur_r, r);
            }
        }
        if !first {
            ans += (cur_r as i128 - cur_l as i128 + 1);
        }
    }

    // Output fits in signed 64-bit under preconditions; print as i128 to be safe.
    println!("{}", ans);
}
