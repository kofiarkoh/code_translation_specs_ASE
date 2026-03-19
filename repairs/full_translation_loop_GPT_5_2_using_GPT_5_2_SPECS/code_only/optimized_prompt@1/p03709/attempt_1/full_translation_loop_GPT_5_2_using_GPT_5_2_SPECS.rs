use std::cmp::{max, min};
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const VERYBIG: i64 = 20_000_000_000_000_007;
const N_MAX: usize = 1_048_576;
const BASE: usize = 262_144; // as in C

#[derive(Clone, Copy, Debug)]
struct Hwll {
    a: i64,
    b: i64,
}

fn bitpow(mut a: u64, mut x: u64, modulo: u64) -> u64 {
    let mut result: u64 = 1;
    while x != 0 {
        if (x & 1) != 0 {
            result = (result * a) % modulo;
        }
        x >>= 1;
        a = (a * a) % modulo;
    }
    result
}

fn solve(n: usize, a: &mut [i64], b: &mut [i64]) -> u64 {
    // Arrays sized to match C globals (but allocated locally).
    let mut c = vec![0i64; n.max(1)];
    let mut d = vec![0i64; n.max(1)];
    let mut e = vec![-1i64; n + 2 + 1]; // indices 0..=n+2 used

    let mut xy = vec![Hwll { a: 0, b: 0 }; n];

    // ssize is computed in C but never used in the active code; still compute to preserve behavior.
    let mut ssize = vec![0u64; BASE * 2];
    for i in (1..=(BASE * 2 - 1)).rev() {
        let v = if i < BASE {
            (ssize[i * 2] + ssize[i * 2 + 1]) % MOD
        } else {
            1
        };
        ssize[i] = v;
    }
    let _ = ssize; // silence unused warning; kept for semantic parity.

    // xy[i] = {a[i], b[i]}; sort by (a,b); write back to a,b
    for i in 0..n {
        xy[i] = Hwll { a: a[i], b: b[i] };
    }
    xy.sort_by(|l, r| (l.a, l.b).cmp(&(r.a, r.b)));
    for i in 0..n {
        a[i] = xy[i].a;
        b[i] = xy[i].b;
    }

    // prefix max of b into c
    for i in 0..n {
        let prev = if i > 0 { c[i - 1] } else { 0 };
        c[i] = max(prev, b[i]);
    }
    // suffix min of b into d
    for i in (0..n).rev() {
        let prev = if i + 1 < n { d[i + 1] } else { VERYBIG };
        d[i] = min(prev, b[i]);
    }

    // compute interval endpoints into xy[i].a/xy[i].b (as indices)
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

    // sort xy by (a,b) again
    xy.sort_by(|l, r| (l.a, l.b).cmp(&(r.a, r.b)));

    // initialize e to -1, then set e[xy[i].b+1] = xy[i].a+1
    for v in e.iter_mut() {
        *v = -1;
    }
    for i in 0..n {
        let aa = xy[i].a + 1;
        let bb = xy[i].b + 1;
        xy[i].a = aa;
        xy[i].b = bb;
        e[bb as usize] = aa;
    }

    // dp and ss: in C they are huge globals dp[N_MAX][4], ss[N_MAX][4]
    // Here allocate only (n+1) rows.
    let mut dp = vec![[0u64; 4]; n + 1];
    let mut ss = vec![[0u64; 4]; n + 1];

    let inv2: u64 = (MOD + 1) / 2;
    let pow2n = bitpow(2, n as u64, MOD);
    dp[0][0] = pow2n;
    ss[0][0] = pow2n;
    dp[0][2] = pow2n;
    ss[0][2] = pow2n;

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i] as usize;
            let r = i;

            for ki in 0..2usize {
                // dp[i][ki] = ss[l-1][1-ki] * inv2^(r-l+1) + (MOD + ss[r-1][3-ki] - ss[l-1][3-ki]) * inv2^r
                let term1 = ss[l - 1][1 - ki]
                    * bitpow(inv2, (r - l + 1) as u64, MOD)
                    % MOD;

                let diff = (MOD + ss[r - 1][3 - ki] + MOD - ss[l - 1][3 - ki]) % MOD;
                let term2 = diff * bitpow(inv2, r as u64, MOD) % MOD;

                let v = (term1 + term2) % MOD;
                dp[i][ki] = v;
                dp[i][2 + ki] = v * bitpow(2, i as u64, MOD) % MOD;
            }
        }

        for ki in 0..4usize {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    (MOD + ss[n][0] + MOD - ss[n][1]) % MOD
}

fn main() {
    // Read all input, whitespace-separated, like scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C has: n=4; m=0; then scanf("%llu", &n, &m); (format bug: only one %llu but two args)
    // In practice, it reads exactly one integer into n; m remains 0.
    // We emulate that: read one u64 if present; otherwise keep default n=4.
    let mut n_u64: u64 = 4;
    if let Some(tok) = it.next() {
        n_u64 = tok.parse::<u64>().unwrap_or(4);
    }
    let n = n_u64 as usize;

    // Then reads n pairs of (a[i], b[i]) as signed long long.
    let mut a = vec![0i64; n];
    let mut b = vec![0i64; n];
    for i in 0..n {
        let ta = it.next().unwrap_or("0").parse::<i64>().unwrap_or(0);
        let tb = it.next().unwrap_or("0").parse::<i64>().unwrap_or(0);
        a[i] = ta;
        b[i] = tb;
    }

    let ans = solve(n, &mut a, &mut b);
    // C prints with "%lld\n" but result is computed mod MOD and fits in signed 64-bit.
    println!("{}", ans as i64);
}
