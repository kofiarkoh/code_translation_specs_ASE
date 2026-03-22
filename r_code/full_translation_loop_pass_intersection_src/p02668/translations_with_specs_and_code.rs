use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read exactly two signed integers (undefined behavior if missing/invalid per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
    let m: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;

    // Spec preconditions: n <= 5049, m <= 5049, m >= 1, n >= 0.
    // We do not add extra error handling; behavior is undefined if violated.

    // Precompute powers of 2 and powers of inv(2) up to n+m (inclusive).
    let inv2_const: i64 = (MOD + 1) / 2; // modular inverse of 2 mod MOD
    let limit = n + m;
    let mut pow2 = vec![0i64; limit + 1];
    let mut inv2 = vec![0i64; limit + 1];
    pow2[0] = 1;
    inv2[0] = 1;
    for i in 1..=limit {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        inv2[i] = (inv2[i - 1] * inv2_const) % MOD;
    }

    // DP tables:
    // DP[i][j] for i in 0..=n, j in 0..=m (we only use j>=1)
    // DQ[i][j] for i in 0..=n, j in 0..=m with DQ[i][0]=0
    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    let mut dq = vec![vec![0i64; m + 1]; n + 1];

    for i in 0..=n {
        // dq[i][0] is already 0 by initialization (invariant).
        for j in 1..=m {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                // DP[i][j] = ((j+1)*DP[i-1][j] + DQ[i-1][j-1]*2^(j-1)) mod MOD
                let term1 = ((j as i64 + 1) * dp[i - 1][j]) % MOD;
                let term2 = (dq[i - 1][j - 1] * pow2[j - 1]) % MOD;
                dp[i][j] = (term1 + term2) % MOD;
            }

            // DQ[i][j] = (DQ[i][j-1] + DP[i][j]*j*(1/2)^j) mod MOD
            let add = (((dp[i][j] * (j as i64)) % MOD) * inv2[j]) % MOD;
            dq[i][j] = (dq[i][j - 1] + add) % MOD;
        }
    }

    let result = dp[n][m] % MOD;
    println!("{result}");
}
