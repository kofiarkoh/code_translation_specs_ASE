use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::io::{self, Read};

fn floor_div(a: i64, b: i64) -> i64 {
    // b must be > 0
    let mut q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q -= 1;
    }
    q
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let x: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Handle specified special cases exactly.
    if x == 0 && d == 0 {
        println!("1");
        return;
    }
    if d == 0 {
        // Possible sums are k*X for k=0..N; distinct count is N+1 (as specified).
        println!("{}", n + 1);
        return;
    }
    if x == 0 {
        // Distinct subset sums of {0, D, 2D, ..., (N-1)D} are {D * t | t in [0..N(N-1)/2]}.
        // Count is N(N-1)/2 + 1 (as specified).
        let nn = n as i128;
        let ans = (nn * (nn - 1) / 2 + 1) as i128;
        println!("{}", ans);
        return;
    }

    // General case:
    // Any subset sum can be written as:
    //   sum_{k in S} (X + kD) = |S|*X + D * sum_{k in S} k
    // Let m = |S| (0..N). For fixed m, sum_{k in S} k ranges over an interval [L(m), R(m)]
    // where:
    //   L(m) = 0 + 1 + ... + (m-1) = m(m-1)/2
    //   R(m) = (N-m) + ... + (N-1) = m(2N-m-1)/2
    // Thus sums for fixed m form an arithmetic progression with step D over that interval.
    //
    // To count distinct sums across all m, group by residue modulo |D|:
    //   value = m*X + D*t
    // For fixed m, all values share residue (m*X mod |D|).
    // For each residue r, we collect intervals of "quotients" q = floor(value/|D|) (with D normalized positive),
    // then union their lengths.

    let mut dd: i64 = d;
    let mut xx: i64 = x;
    if dd < 0 {
        dd = -dd;
        xx = -xx;
    }
    let dd_i128 = dd as i128;

    // Map residue -> list of intervals [l, r] in quotient space (inclusive).
    let mut mp: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

    for m in 0..=n {
        let m_i128 = m as i128;
        let n_i128 = n as i128;

        let l_t = m_i128 * (m_i128 - 1) / 2;
        let r_t = m_i128 * (2 * n_i128 - m_i128 - 1) / 2;

        // base = m*X
        let base = m_i128 * (xx as i128);

        // residue modulo dd in [0, dd-1]
        let mut r = (base % dd_i128) as i64;
        if r < 0 {
            r += dd;
        }

        // For t in [l_t, r_t], value = base + dd*t.
        // Quotient q = floor(value / dd) = floor(base/dd) + t (since dd>0).
        let base_q = floor_div(base as i64, dd);
        let l_q = base_q + l_t as i64;
        let r_q = base_q + r_t as i64;

        mp.entry(r).or_default().push((l_q, r_q));
    }

    let mut ans: i128 = 0;

    for (_r, mut intervals) in mp {
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
                ans += (cur_r as i128 - cur_l as i128 + 1);
                cur_l = l;
                cur_r = r;
            } else {
                cur_r = max(cur_r, r);
                cur_l = min(cur_l, l);
            }
        }
        if !first {
            ans += (cur_r as i128 - cur_l as i128 + 1);
        }
    }

    println!("{}", ans);
}
