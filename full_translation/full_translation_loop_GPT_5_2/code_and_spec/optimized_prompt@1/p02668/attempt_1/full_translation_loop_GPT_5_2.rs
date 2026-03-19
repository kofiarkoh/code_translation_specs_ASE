use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read exactly two integers n and m from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next().and_then(|s| s.parse::<i64>().ok());
    let m_opt = it.next().and_then(|s| s.parse::<i64>().ok());

    // Spec: if invalid input conditions, output -1.
    let (n, m) = match (n_opt, m_opt) {
        (Some(n), Some(m)) => (n, m),
        _ => {
            print!("-1\n");
            return;
        }
    };

    // Spec precondition: n and m must be non-negative.
    if n < 0 || m < 0 {
        print!("-1\n");
        return;
    }

    // The original C code uses fixed-size dp/dq arrays [5050][5050].
    // Enforce bounds to avoid OOM and to match "maximum size constraints".
    // Note: C would have UB/out-of-bounds if exceeded; spec says output -1 on invalid.
    const LIM: usize = 5050;
    let n_us = n as usize;
    let m_us = m as usize;
    if n_us >= LIM || m_us >= LIM {
        print!("-1\n");
        return;
    }

    // Precompute pow2 and inv2 up to n+m (inclusive), as in C.
    let nm = n_us + m_us;
    let inv: i64 = (MOD + 1) / 2; // modular inverse of 2 mod MOD

    let mut pow2 = vec![0i64; nm + 1];
    let mut inv2 = vec![0i64; nm + 1];
    pow2[0] = 1;
    inv2[0] = 1;
    for i in 1..=nm {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        inv2[i] = (inv2[i - 1] * inv) % MOD;
    }

    // dp and dq tables.
    let mut dp = vec![vec![0i64; m_us + 1]; n_us + 1];
    let mut dq = vec![vec![0i64; m_us + 1]; n_us + 1];

    // The C loops:
    // for i=0..=n:
    //   for j=1..=m:
    //     if i==0: dp=1 else dp=((j+1)*dp[i-1][j] + dq[i-1][j-1]*pow2[j-1])%MOD
    //     dq[i][j] = (dq[i][j-1] + (dp[i][j]*j%MOD)*inv2[j])%MOD
    for i in 0..=n_us {
        for j in 1..=m_us {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                let term1 = ((j as i64 + 1) * dp[i - 1][j]) % MOD;
                let term2 = (dq[i - 1][j - 1] * pow2[j - 1]) % MOD;
                dp[i][j] = (term1 + term2) % MOD;
            }

            let add = ((dp[i][j] * j as i64) % MOD) * inv2[j] % MOD;
            dq[i][j] = (dq[i][j - 1] + add) % MOD;
        }
    }

    // Output dp[n][m] with newline, exactly like printf("%lld\n", result);
    print!("{}\n", dp[n_us][m_us]);
}
