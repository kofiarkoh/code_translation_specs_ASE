use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, parse two signed integers (like scanf("%lld%lld", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // The specification notes undefined behavior for negative/out-of-bounds inputs.
    // In Rust, we must avoid UB; we choose the narrowest reasonable interpretation:
    // only proceed for nonnegative n,m that fit in usize, otherwise do nothing meaningful.
    // (The original C would be undefined; here we avoid panics/UB.)
    if n < 0 || m < 0 {
        // No defined behavior; exit quietly.
        return;
    }
    let n_us = n as usize;
    let m_us = m as usize;

    let nm = n_us + m_us;

    // Precompute pow2 and inv2 for x = 0..(n+m).
    let mut pow2 = vec![0i64; nm + 1];
    let mut inv2 = vec![0i64; nm + 1];
    pow2[0] = 1;
    inv2[0] = 1;
    let half = (MOD + 1) / 2; // modular inverse of 2 mod MOD
    for x in 1..=nm {
        pow2[x] = (pow2[x - 1] * 2) % MOD;
        inv2[x] = (inv2[x - 1] * half) % MOD;
    }

    // dp and dq are conceptually global zero-initialized tables.
    // We allocate exactly (n+1) x (m+1) and rely on zero initialization.
    let mut dp = vec![vec![0i64; m_us + 1]; n_us + 1];
    let mut dq = vec![vec![0i64; m_us + 1]; n_us + 1];

    // Fill order: i = 0..n, j = 1..m.
    for i in 0..=n_us {
        for j in 1..=m_us {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                let term1 = ((j as i64 + 1) % MOD) * dp[i - 1][j] % MOD;
                let term2 = dq[i - 1][j - 1] * pow2[j - 1] % MOD;
                dp[i][j] = (term1 + term2) % MOD;
            }

            // dq[i][0] remains 0 due to initialization; update dq[i][j].
            let add = (dp[i][j] * (j as i64 % MOD) % MOD) * inv2[j] % MOD;
            dq[i][j] = (dq[i][j - 1] + add) % MOD;
        }
    }

    // Print dp[n][m] modulo MOD as signed decimal with newline.
    // For m == 0, dp[*][0] remains 0 due to initialization, matching the spec.
    println!("{}", dp[n_us][m_us] % MOD);
}
