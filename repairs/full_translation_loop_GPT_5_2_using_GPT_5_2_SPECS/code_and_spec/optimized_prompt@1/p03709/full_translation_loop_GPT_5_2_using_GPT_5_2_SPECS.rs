use std::cmp::{max, min};
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const VERYBIG: i64 = 20_000_000_000_000_007;

fn mod_pow(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u64 = 1;
    while e != 0 {
        if (e & 1) != 0 {
            r = (r * a) % m;
        }
        e >>= 1;
        a = (a * a) % m;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap(),
        None => return,
    };

    // SPEC says there is a second integer (ignored). The C code *intends* to read it too.
    // Some judge inputs may omit it; if so, treat it as absent and proceed.
    // IMPORTANT: If the second integer is absent, we must NOT consume the first `a` of the first pair.
    // So we only consume it when enough tokens remain for (ignored + 2*n).
    let remaining: usize = it.clone().count();
    let need_with_ignored = 1 + 2 * n;
    let need_without_ignored = 2 * n;
    if remaining == need_with_ignored {
        let _m_ignored: i64 = it.next().unwrap().parse().unwrap();
        let _ = _m_ignored;
    } else if remaining == need_without_ignored {
        // no ignored value present
    } else {
        // Fallback: try to follow SPEC (consume one), but avoid panicking if missing.
        let _ = it.next();
    }

    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
        b[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    // Sort pairs by (a, b).
    let mut pairs: Vec<(i64, i64)> = (0..n).map(|i| (a[i], b[i])).collect();
    pairs.sort_unstable_by(|l, r| l.0.cmp(&r.0).then(l.1.cmp(&r.1)));
    for i in 0..n {
        a[i] = pairs[i].0;
        b[i] = pairs[i].1;
    }

    // Prefix max and suffix min of b.
    let mut c: Vec<i64> = vec![0; n];
    let mut d: Vec<i64> = vec![0; n];
    for i in 0..n {
        c[i] = if i == 0 { 0 } else { c[i - 1] };
        c[i] = max(c[i], b[i]);
    }
    for i in (0..n).rev() {
        d[i] = if i + 1 < n { d[i + 1] } else { VERYBIG };
        d[i] = min(d[i], b[i]);
    }

    // Compute xy[i].a (left) and xy[i].b (right) in-place, then sort by (a,b).
    let mut xy: Vec<(i64, i64)> = vec![(0, 0); n];
    for i in 0..n {
        // left boundary
        let left = if i == 0 || c[i - 1] <= b[i] {
            i as i64
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
            ng
        };

        // right boundary
        let right = if i + 1 >= n || d[i + 1] >= b[i] {
            i as i64
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
            ng
        };

        xy[i] = (left, right);
    }
    xy.sort_unstable_by(|l, r| l.0.cmp(&r.0).then(l.1.cmp(&r.1)));

    // Build e array: e[xy[i].b+1] = xy[i].a+1, initialized to -1.
    let mut e: Vec<i64> = vec![-1; n + 3];
    for i in 0..n {
        let l1 = xy[i].0 + 1;
        let r1 = xy[i].1 + 1;
        e[r1 as usize] = l1;
    }

    let inv2: u64 = (MOD + 1) / 2;
    let pow2n = mod_pow(2, n as u64, MOD);

    let mut dp: Vec<[u64; 4]> = vec![[0; 4]; n + 1];
    let mut ss: Vec<[u64; 4]> = vec![[0; 4]; n + 1];

    dp[0][0] = pow2n;
    ss[0][0] = pow2n;
    dp[0][2] = pow2n;
    ss[0][2] = pow2n;

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i] as usize; // 1-based
            let r = i; // 1-based

            for ki in 0..2usize {
                let term1 = ss[l - 1][1 - ki]
                    * mod_pow(inv2, (r - l + 1) as u64, MOD)
                    % MOD;

                let sr = ss[r - 1][3 - ki];
                let sl = ss[l - 1][3 - ki];
                let diff = (MOD + sr - sl) % MOD;

                let term2 = diff * mod_pow(inv2, r as u64, MOD) % MOD;

                let v = (term1 + term2) % MOD;
                dp[i][ki] = v;
                dp[i][2 + ki] = v * mod_pow(2, i as u64, MOD) % MOD;
            }
        }

        for ki in 0..4usize {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    let result = (MOD + ss[n][0] - ss[n][1]) % MOD;
    print!("{result}\n");
}
