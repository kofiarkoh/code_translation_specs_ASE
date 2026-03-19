use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, parse two integers like scanf("%d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap_or("0").parse().unwrap();
    let m: i32 = it.next().unwrap_or("0").parse().unwrap();

    // C: long long dp[302][302] = {}, tmp;
    // Use i64. Indices are within [0..=301] for typical constraints; we keep 302x302.
    let mut dp = vec![vec![0i64; 302]; 302];

    // C: for (k = 1, dp[1][1] = 1; k <= M; k++)
    dp[1][1] = 1;
    for k in 1..=m {
        // for (i = k; i >= 1; i--)
        let mut i = k;
        while i >= 1 {
            // for (j = i; j >= 1; j--)
            let mut j = i;
            while j >= 1 {
                let ii = i as usize;
                let jj = j as usize;

                // C: tmp = dp[i][j] % Mod; dp[i][j] = 0;
                // In C, % keeps the sign of the dividend; dp values here are expected non-negative,
                // but we preserve the same operator behavior.
                let tmp = dp[ii][jj] % MOD;
                dp[ii][jj] = 0;

                // C arithmetic is in long long; we do i64 with explicit casts.
                let n_minus_i = (n - i) as i64;
                let j_i64 = j as i64;
                let i_minus_j = (i - j) as i64;

                // dp[i+1][j] += tmp * (N - i);
                dp[(i + 1) as usize][jj] += tmp * n_minus_i;
                // dp[i][i] += tmp * j;
                dp[ii][ii] += tmp * j_i64;
                // dp[i][j] += tmp * (i - j);
                dp[ii][jj] += tmp * i_minus_j;

                j -= 1;
            }
            i -= 1;
        }
    }

    // printf("%lld\n", dp[N][N] % Mod);
    let ans = dp[n as usize][n as usize] % MOD;
    print!("{ans}\n");
    // fflush(stdout); not necessary in Rust for normal program exit, but printing is immediate enough.
}
