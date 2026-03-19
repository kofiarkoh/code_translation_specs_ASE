use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const M: usize = 1 << 20;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    // Iterative fast exponentiation (matches C's recursive power semantics for nonnegative e).
    a = mod_norm(a);
    let mut res: i64 = 1;
    while e > 0 {
        if (e & 1) == 1 {
            res = mod_norm(res * a);
        }
        a = mod_norm(a * a);
        e >>= 1;
    }
    res
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;

    // Arrays sized N+1, 1-indexed for a, s, onum
    let mut s: Vec<usize> = vec![0; n + 1];
    let mut onum: Vec<i64> = vec![0; n + 1];

    for i in 1..=n {
        let ai: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
        s[i] = s[i - 1] ^ ai;
        onum[i] = onum[i - 1] + if s[i] == 0 { 1 } else { 0 };
    }

    // State arrays keyed by prefix-xor value in [0, 2^20)
    let mut prev: Vec<usize> = vec![0; M];
    let mut dp: Vec<i64> = vec![0; M];
    let mut sum: Vec<i64> = vec![0; M];
    let mut osum: Vec<i64> = vec![1; M];

    for i in 1..=n {
        let x = s[i];
        let delta_zeros = onum[i] - onum[prev[x]];
        dp[x] = mod_norm(sum[x] * delta_zeros + osum[x]);
        osum[x] = mod_norm(osum[x] + sum[x] * delta_zeros);
        sum[x] = mod_norm(sum[x] + dp[x]);
        prev[x] = i;
    }

    let ans: i64 = if s[n] == 0 {
        // Per C code: power(2, Onum[N] - 1) + sum over nonzero xor values.
        // Note: If Onum[N] == 0, exponent is -1; spec preconditions do not rule this out,
        // but the original C would recurse infinitely. We preserve behavior only for valid inputs.
        let mut a = mod_pow(2, onum[n] - 1);
        for i in 1..M {
            a = mod_norm(a + sum[i]);
        }
        a
    } else {
        dp[s[n]]
    };

    print!("{ans}\n");
}
