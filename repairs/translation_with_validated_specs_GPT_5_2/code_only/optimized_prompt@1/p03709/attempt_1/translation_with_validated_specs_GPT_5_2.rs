use std::cmp::{max, min};
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const VERYBIG: i64 = 20_000_000_000_000_007;
const N_MAX: usize = 1_048_576;
const BASE: usize = 262_144;

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

fn solve(n: usize, a: &mut [i64], b: &mut [i64]) -> i64 {
    // Arrays sized like the C globals (but allocated locally).
    let mut xy: Vec<Hwll> = vec![Hwll { a: 0, b: 0 }; n];
    let mut c: Vec<i64> = vec![0; n];
    let mut d: Vec<i64> = vec![0; n];
    let mut e: Vec<i64> = vec![-1; n + 2];

    // dp/ss are huge in C: dp[N_MAX][4], ss[N_MAX][4].
    // We only need up to n inclusive.
    let mut dp: Vec<[u64; 4]> = vec![[0; 4]; n + 1];
    let mut ss: Vec<[u64; 4]> = vec![[0; 4]; n + 1];

    // ssize is computed but never used later in the active code; still emulate.
    let mut ssize: Vec<u64> = vec![0; BASE * 2];
    for i in (1..=(BASE * 2 - 1)).rev() {
        let v = if i < BASE {
            (ssize[i * 2] + ssize[i * 2 + 1]) % MOD
        } else {
            1
        };
        ssize[i] = v;
    }

    // xy[i] = {a[i], b[i]}; sort by (a,b)
    for i in 0..n {
        xy[i] = Hwll { a: a[i], b: b[i] };
    }
    xy.sort_by(|l, r| l.a.cmp(&r.a).then(l.b.cmp(&r.b)));
    for i in 0..n {
        a[i] = xy[i].a;
        b[i] = xy[i].b;
    }

    // prefix max of b into c
    for i in 0..n {
        c[i] = max(if i > 0 { c[i - 1] } else { 0 }, b[i]);
    }
    // suffix min of b into d
    for i in (0..n).rev() {
        d[i] = min(if i + 1 < n { d[i + 1] } else { VERYBIG }, b[i]);
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

    // sort by (a,b) again
    xy.sort_by(|l, r| l.a.cmp(&r.a).then(l.b.cmp(&r.b)));

    // e[xy[i].b+1] = xy[i].a+1 (after increment)
    for i in 0..n {
        xy[i].a += 1;
        xy[i].b += 1;
        let idx = xy[i].b as usize;
        e[idx] = xy[i].a;
    }

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
                // dp[i][ki] = ss[l-1][1-ki] * inv2^(r-l+1)
                let term1 = ss[l - 1][1 - ki]
                    * bitpow(inv2, (r - l + 1) as u64, MOD)
                    % MOD;

                // dp[i][ki] += (MOD + ss[r-1][3-ki] - ss[l-1][3-ki]) * inv2^r
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

    let result = ((MOD + ss[n][0] + MOD - ss[n][1]) % MOD) as i64;
    result
}

fn main() {
    // Read all input; emulate scanf whitespace behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code: scanf("%llu", &n, &m); (format mismatch, but it reads two numbers)
    // We'll read two u64 if present; if missing, default 0.
    let n_u64: u64 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let _m_u64: u64 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let n = n_u64 as usize;

    let mut a = vec![0i64; n];
    let mut b = vec![0i64; n];
    for i in 0..n {
        a[i] = it.next().unwrap_or("0").parse().unwrap_or(0);
        b[i] = it.next().unwrap_or("0").parse().unwrap_or(0);
    }

    let ans = solve(n, &mut a, &mut b);
    print!("{ans}\n");
}
