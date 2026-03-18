use std::cmp::{max, min};
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const VERYBIG: i64 = 20_000_000_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Pair {
    a: i64,
    b: i64,
}

fn mod_pow(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u64 = 1;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % m;
        }
        e >>= 1;
        a = (a * a) % m;
    }
    r
}

fn main() {
    // The original C has UB in input parsing (scanf format mismatch).
    // To match judge inputs robustly, accept either:
    //   (1) n then n pairs
    //   (2) n m then n pairs
    // We detect which by token count.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let toks: Vec<&str> = input.split_whitespace().collect();
    if toks.is_empty() {
        return;
    }

    let n: usize = toks[0].parse().unwrap();

    // Determine where pairs start.
    // If total tokens == 1 + 2*n => no m
    // If total tokens == 2 + 2*n => has m (ignored)
    // Otherwise, fall back: if there are enough tokens for m+pairs, assume m exists.
    let start = if toks.len() == 1 + 2 * n {
        1
    } else if toks.len() == 2 + 2 * n {
        2
    } else if toks.len() >= 2 + 2 * n {
        2
    } else {
        // Not enough tokens; assume no m and parse what we can (but judge inputs are valid).
        1
    };

    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; n];
    for i in 0..n {
        let idx = start + 2 * i;
        a[i] = toks[idx].parse().unwrap();
        b[i] = toks[idx + 1].parse().unwrap();
    }

    // --- solve() ---
    let mut xy: Vec<Pair> = (0..n).map(|i| Pair { a: a[i], b: b[i] }).collect();
    xy.sort_by(|l, r| l.a.cmp(&r.a).then(l.b.cmp(&r.b)));
    for i in 0..n {
        a[i] = xy[i].a;
        b[i] = xy[i].b;
    }

    let mut c: Vec<i64> = vec![0; n];
    for i in 0..n {
        c[i] = max(if i > 0 { c[i - 1] } else { 0 }, b[i]);
    }

    let mut d: Vec<i64> = vec![0; n];
    for i in (0..n).rev() {
        d[i] = min(if i + 1 < n { d[i + 1] } else { VERYBIG }, b[i]);
    }

    // Reuse xy to store computed interval endpoints (as in C).
    for i in 0..n {
        // left endpoint
        if i == 0 || c[i - 1] <= b[i] {
            xy[i].a = i as i64;
        } else {
            let mut ok: i64 = -1;
            let mut ng: i64 = i as i64;
            while ok + 1 < ng {
                let med = (ok + ng) / 2;
                if c[med as usize] > b[i] {
                    ng = med;
                } else {
                    ok = med;
                }
            }
            xy[i].a = ng;
        }

        // right endpoint
        if i + 1 == n || d[i + 1] >= b[i] {
            xy[i].b = i as i64;
        } else {
            let mut ok: i64 = n as i64;
            let mut ng: i64 = i as i64;
            while ng + 1 < ok {
                let med = (ok + ng) / 2;
                if d[med as usize] < b[i] {
                    ng = med;
                } else {
                    ok = med;
                }
            }
            xy[i].b = ng;
        }
    }

    xy.sort_by(|l, r| l.a.cmp(&r.a).then(l.b.cmp(&r.b)));

    // e indexed 0..=n+1, initialized to -1
    let mut e: Vec<i64> = vec![-1; n + 2];
    for i in 0..n {
        // shift to 1-based indices
        xy[i].a += 1;
        xy[i].b += 1;
        e[xy[i].b as usize] = xy[i].a;
    }

    // dp and prefix sums ss: (n+1) x 4
    let mut dp: Vec<[u64; 4]> = vec![[0; 4]; n + 1];
    let mut ss: Vec<[u64; 4]> = vec![[0; 4]; n + 1];

    let inv2: u64 = (MOD + 1) / 2;
    let pow2n = mod_pow(2, n as u64, MOD);
    dp[0][0] = pow2n;
    ss[0][0] = pow2n;
    dp[0][2] = pow2n;
    ss[0][2] = pow2n;

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i] as usize;
            let r = i;

            for ki in 0..2usize {
                let term1 = ss[l - 1][1 - ki]
                    * mod_pow(inv2, (r - l + 1) as u64, MOD)
                    % MOD;

                // (MOD + ss[r-1][3-ki] - ss[l-1][3-ki]) % MOD
                let diff = (ss[r - 1][3 - ki] + MOD - ss[l - 1][3 - ki]) % MOD;
                let term2 = diff * mod_pow(inv2, r as u64, MOD) % MOD;

                let val = (term1 + term2) % MOD;
                dp[i][ki] = val;
                dp[i][2 + ki] = val * mod_pow(2, i as u64, MOD) % MOD;
            }
        }

        for ki in 0..4usize {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    let result = (ss[n][0] + MOD - ss[n][1]) % MOD;

    // The provided C code prints only the numeric result.
    println!("{}", result);
}
