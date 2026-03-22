use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut s = vec![0i32; n];
    let mut t = vec![0i32; m];

    for i in 0..n {
        s[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 0..m {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // dp[i][j] for 0..=n, 0..=m
    let mut dp = vec![vec![0i64; m + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                dp[i][j] = 1;
                continue;
            }
            let mut val = (dp[i - 1][j] + dp[i][j - 1]) % MOD;
            if s[i - 1] != t[j - 1] {
                // Subtract dp[i-1][j-1] in modulo world, matching C's behavior:
                // dp can go negative, then % keeps sign, then we add MOD if negative.
                val -= dp[i - 1][j - 1];
                val %= MOD;
                if val < 0 {
                    val += MOD;
                }
            }
            dp[i][j] = val;
        }
    }

    print!("{}\n", dp[n][m]);
}
