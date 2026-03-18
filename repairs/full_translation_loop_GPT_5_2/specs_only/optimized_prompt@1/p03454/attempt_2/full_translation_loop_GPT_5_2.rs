use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a = mod_norm(a);
    let mut r: i64 = 1;
    while e > 0 {
        if (e & 1) != 0 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    // MOD is prime in typical competitive settings; spec only says "modulo a constant".
    // We use Fermat inverse; if MOD were not prime, behavior would be unspecified anyway.
    mod_pow(a, MOD - 2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap_or("0").parse().unwrap();

    // Preconditions: n must be non-negative; otherwise undefined by spec.
    if n < 0 {
        return;
    }

    // The spec is vague, but the failing sample matches the classic count of integer partitions
    // of n into parts of size at least 3 (order does not matter), modulo 1e9+7.
    //
    // This is equivalent to the number of partitions of n - 3k into any parts, summed over k,
    // but we compute directly with DP for partitions with minimum part size 3:
    // dp[s] = number of ways to write s as sum of integers >= 3, order-independent.
    //
    // Unbounded knapsack / coin-change with coin values 3..=n.
    let nn = n as usize;
    let mut dp = vec![0i64; nn + 1];
    dp[0] = 1;
    for coin in 3..=nn {
        for s in coin..=nn {
            let v = dp[s] + dp[s - coin];
            dp[s] = mod_norm(v);
        }
    }

    // Ensure output in [0, MOD).
    let ans = mod_norm(dp[nn]);
    print!("{}", ans);
}
