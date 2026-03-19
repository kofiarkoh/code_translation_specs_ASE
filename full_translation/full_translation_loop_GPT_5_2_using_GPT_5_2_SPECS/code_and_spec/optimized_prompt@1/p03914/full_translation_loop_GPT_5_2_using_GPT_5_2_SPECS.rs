use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, parse two integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // Fixed-size DP table as in C: dp[302][302], zero-initialized.
    // Use i64 to match C's long long.
    let mut dp = vec![vec![0i64; 302]; 302];

    // C initializes dp[1][1] = 1 and runs k from 1..=M.
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

                let tmp = dp[ii][jj] % MOD;
                dp[ii][jj] = 0;

                // dp[i+1][j] += tmp * (N - i);
                // dp[i][i]   += tmp * j;
                // dp[i][j]   += tmp * (i - j);
                //
                // Note: This intentionally does not reduce modulo after each update,
                // matching the C code's behavior (overflow is undefined if preconditions
                // are violated).
                dp[(i + 1) as usize][jj] += tmp * (n - i) as i64;
                dp[ii][ii] += tmp * j as i64;
                dp[ii][jj] += tmp * (i - j) as i64;

                j -= 1;
            }
            i -= 1;
        }
    }

    // Output dp[N][N] % MOD with a trailing newline, matching C.
    let nn = n as usize;
    let ans = dp[nn][nn] % MOD;
    println!("{ans}");
}
