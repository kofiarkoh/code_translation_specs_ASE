use std::io::{self, Read};

const P: i64 = 1_000_000_007;
const M: usize = 1 << 20;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    base %= P;
    let mut res: i64 = 1;
    while exp > 0 {
        if (exp & 1) != 0 {
            res = (res * base) % P;
        }
        base = (base * base) % P;
        exp >>= 1;
    }
    res
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Prefix-xor sequence s_i is maintained incrementally; s_0 = 0.
    // Onum[i] counts how many s_j == 0 for j in 1..=i; Onum[0] = 0.
    let mut onum: Vec<i64> = vec![0; n + 1];

    // Per xor-class state.
    let mut prev: Vec<usize> = vec![0; M];
    let mut dp: Vec<i64> = vec![0; M];
    let mut sum: Vec<i64> = vec![0; M];
    let mut osum: Vec<i64> = vec![1; M];

    let mut s: usize = 0;

    for i in 1..=n {
        // The specification allows negative inputs in the input type, but requires
        // that all prefix-xors remain within [0, 2^20-1]; otherwise behavior is undefined.
        // We parse as i64 then cast to u32 to emulate typical C integer-to-unsigned
        // conversion before XOR, then truncate to usize.
        let a_i: i64 = it.next().unwrap().parse().unwrap();
        let a_u: u32 = a_i as u32;

        s = (s as u32 ^ a_u) as usize;

        onum[i] = onum[i - 1] + if s == 0 { 1 } else { 0 };

        let x = s;
        // Under the spec, x must be in [0, M-1]; otherwise UB in the original.
        // We do not add bounds checks beyond Rust's inherent indexing checks.
        let px = prev[x];

        let delta: i64 = onum[i] - onum[px];

        let sum_x = sum[x];
        let osum_x = osum[x];

        let dp_new = (sum_x * (delta % P) + osum_x) % P;
        dp[x] = dp_new;

        osum[x] = (osum_x + sum_x * (delta % P)) % P;
        sum[x] = (sum_x + dp_new) % P;

        prev[x] = i;
    }

    let t = s;
    let ans: i64 = if t != 0 {
        dp[t]
    } else {
        // When s_N == 0, Onum[N] >= 1, so exponent is nonnegative.
        let exp = onum[n] - 1;
        let mut total = mod_pow(2, exp);

        // Sum over x = 1..M-1 of sum[x].
        for x in 1..M {
            total += sum[x];
            if total >= P {
                total -= P;
            }
        }
        total % P
    };

    println!("{}", ans);
}
