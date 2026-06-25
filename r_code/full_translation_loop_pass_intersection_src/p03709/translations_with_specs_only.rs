use std::cmp::{max, min};
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const INV2: i64 = (MOD + 1) / 2;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_add(a: i64, b: i64) -> i64 {
    let mut s = a + b;
    if s >= MOD {
        s -= MOD;
    }
    s
}

fn mod_sub(a: i64, b: i64) -> i64 {
    let mut d = a - b;
    if d < 0 {
        d += MOD;
    }
    d
}

fn mod_mul(a: i64, b: i64) -> i64 {
    ((a as i128 * b as i128) % (MOD as i128)) as i64
}

fn mod_pow(mut a: i64, mut e: u64) -> i64 {
    let mut r: i64 = 1;
    while e > 0 {
        if (e & 1) != 0 {
            r = mod_mul(r, a);
        }
        a = mod_mul(a, a);
        e >>= 1;
    }
    r
}

fn main() {
    // Tokenization semantics: read exactly one unsigned integer token as n.
    // Any extra token after n is part of the (a_i, b_i) stream.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_u: u64 = match it.next() {
        Some(t) => t.parse::<u64>().unwrap(),
        None => return, // Undefined behavior in spec; we just terminate quietly.
    };
    let n: usize = n_u as usize;

    let mut pairs: Vec<(i64, i64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let a = it.next().unwrap().parse::<i64>().unwrap();
        let b = it.next().unwrap().parse::<i64>().unwrap();
        pairs.push((a, b));
    }

    // 5.1 Sorting by (a, b)
    pairs.sort_by(|x, y| x.0.cmp(&y.0).then_with(|| x.1.cmp(&y.1)));

    let mut b: Vec<i64> = Vec::with_capacity(n);
    for &(_, bi) in &pairs {
        b.push(bi);
    }

    // 5.2 Prefix max c and suffix min d over b
    let mut c = vec![0i64; n];
    let mut d = vec![0i64; n];
    if n > 0 {
        c[0] = b[0];
        for i in 1..n {
            c[i] = max(c[i - 1], b[i]);
        }
        d[n - 1] = b[n - 1];
        for i in (0..n - 1).rev() {
            d[i] = min(d[i + 1], b[i]);
        }
    }

    // 5.3 Compute (L_i, R_i), then sort by (L, R), then fill e[r] = l (1-based)
    let mut lr: Vec<(usize, usize)> = Vec::with_capacity(n);
    for i in 0..n {
        // Left boundary L_i
        let li0: usize;
        if i == 0 || c[i - 1] <= b[i] {
            li0 = i;
        } else {
            // smallest j in [0, i] such that c[j] > b[i]
            let mut lo = 0usize;
            let mut hi = i; // inclusive
            while lo < hi {
                let mid = (lo + hi) / 2;
                if c[mid] > b[i] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            li0 = lo;
        }

        // Right boundary R_i
        let ri0: usize;
        if i == n - 1 || d[i + 1] >= b[i] {
            ri0 = i;
        } else {
            // largest j in [i, n-1] such that d[j] < b[i]
            let mut lo = i;
            let mut hi = n - 1; // inclusive
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if d[mid] < b[i] {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            ri0 = lo;
        }

        lr.push((li0 + 1, ri0 + 1)); // shift to 1-based
    }

    lr.sort_by(|x, y| x.0.cmp(&y.0).then_with(|| x.1.cmp(&y.1)));

    let mut e = vec![-1i32; n + 2]; // indices up to at least n+1
    for (l1, r1) in lr {
        // Overwrite semantics for same r.
        e[r1] = l1 as i32;
    }

    // Precompute powers needed: 2^i and inv2^i for i=0..n
    let mut pow2 = vec![0i64; n + 1];
    let mut powinv2 = vec![0i64; n + 1];
    pow2[0] = 1;
    powinv2[0] = 1;
    for i in 1..=n {
        pow2[i] = mod_add(pow2[i - 1], pow2[i - 1]); // *2 mod MOD
        powinv2[i] = mod_mul(powinv2[i - 1], INV2);
    }

    // 5.4 DP tables dp[i][k], ss[i][k] for i=0..n, k=0..3
    let mut dp = vec![[0i64; 4]; n + 1];
    let mut ss = vec![[0i64; 4]; n + 1];

    let p = pow2[n]; // 2^n mod MOD
    dp[0][0] = p;
    ss[0][0] = p;
    dp[0][2] = p;
    ss[0][2] = p;

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i] as usize; // 1-based
            let r = i; // 1-based
            let len = r - l + 1;

            for ki in 0..=1usize {
                let term1 = mod_mul(ss[l - 1][1 - ki], powinv2[len]);
                let diff = mod_sub(ss[r - 1][3 - ki], ss[l - 1][3 - ki]);
                let term2 = mod_mul(diff, powinv2[r]);
                dp[i][ki] = mod_add(term1, term2);
                dp[i][2 + ki] = mod_mul(dp[i][ki], pow2[i]);
            }
        }
        for k in 0..4 {
            ss[i][k] = mod_add(ss[i - 1][k], dp[i][k]);
        }
    }

    // 5.5 Final result
    let result = mod_sub(ss[n][0], ss[n][1]);
    println!("{}", result);
}
