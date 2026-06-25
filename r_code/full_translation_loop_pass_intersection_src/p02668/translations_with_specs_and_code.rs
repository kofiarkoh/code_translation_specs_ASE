use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read exactly two signed integers n and m from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // SPEC notes: the original C program assumes n,m are within fixed array bounds
    // and does not validate non-negativity. We follow the same assumption.
    // If n or m are negative, behavior is undefined in C; in Rust we'd panic on
    // allocation/indices. This matches "undefined behavior" per spec.
    let n_usize: usize = n.try_into().unwrap();
    let m_usize: usize = m.try_into().unwrap();

    // Precompute pow2 and inv2 for x = 0..(n+m)
    let nm: usize = (n + m).try_into().unwrap();
    let inv: i64 = (MOD + 1) / 2;

    let mut pow2 = vec![0i64; nm + 1];
    let mut inv2 = vec![0i64; nm + 1];
    pow2[0] = 1;
    inv2[0] = 1;
    for x in 1..=nm {
        pow2[x] = (pow2[x - 1] * 2) % MOD;
        inv2[x] = (inv2[x - 1] * inv) % MOD;
    }

    // dp and dq tables for i=0..n, j=0..m, zero-initialized.
    // dq[i][0] = 0 and dp[i][0] = 0 naturally hold.
    let mut dp = vec![vec![0i64; m_usize + 1]; n_usize + 1];
    let mut dq = vec![vec![0i64; m_usize + 1]; n_usize + 1];

    for i in 0..=n_usize {
        for j in 1..=m_usize {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                let jj = j as i64;
                let term1 = ((jj + 1) % MOD) * dp[i - 1][j] % MOD;
                let term2 = dq[i - 1][j - 1] * pow2[j - 1] % MOD;
                dp[i][j] = (term1 + term2) % MOD;
            }

            let jj = j as i64;
            let add = (dp[i][j] * jj % MOD) * inv2[j] % MOD;
            dq[i][j] = (dq[i][j - 1] + add) % MOD;
        }
    }

    let result = dp[n_usize][m_usize] % MOD;
    print!("{result}\n");
}
