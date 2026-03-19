use std::io::{self, Read};

const P: i64 = 1_000_000_007;

fn mod_norm(mut a: i64) -> i64 {
    a %= P;
    if a >= 0 { a } else { a + P }
}

fn power(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        // Match C's behavior: compute a*a in i64 (C: signed overflow is UB, but intended mod-mul).
        power(mod_norm(a.wrapping_mul(a)), n / 2)
    } else {
        mod_norm(a.wrapping_mul(power(a, n - 1)))
    }
}

fn main() {
    // Read all input, parse as i64 like scanf("%lld").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n_usize = n as usize;

    // C: M = ((int)1 << 20) with int=long long => 1LL<<20
    let m: usize = 1usize << 20;

    // Arrays sized N+1, 1-indexed for a/s/Onum.
    let mut a = vec![0i64; n_usize + 1];
    let mut s = vec![0i64; n_usize + 1];
    let mut onum = vec![0i64; n_usize + 1];
    s[0] = 0;
    onum[0] = 0;

    for i in 1..=n_usize {
        let val: i64 = it.next().unwrap().parse().unwrap();
        a[i] = val;
        s[i] = s[i - 1] ^ a[i];
        onum[i] = onum[i - 1] + if s[i] == 0 { 1 } else { 0 };
    }

    // prev/dp/sum/Osum sized M. C initializes prev/dp/sum to 0, Osum to 1.
    let mut prev = vec![0usize; m];
    let mut dp = vec![0i64; m];
    let mut sum = vec![0i64; m];
    let mut osum = vec![1i64; m];

    for i in 1..=n_usize {
        let x_i64 = s[i];
        // In the original code, x is used as an index into arrays of size 2^20.
        // This assumes 0 <= s[i] < M. We preserve that assumption; if violated, C would OOB.
        let x = x_i64 as usize;

        let delta = onum[i] - onum[prev[x]];
        dp[x] = mod_norm(sum[x].wrapping_mul(delta).wrapping_add(osum[x]));
        osum[x] = mod_norm(osum[x].wrapping_add(sum[x].wrapping_mul(delta)));
        sum[x] = mod_norm(sum[x].wrapping_add(dp[x]));
        prev[x] = i;
    }

    let mut ans: i64;
    if s[n_usize] == 0 {
        ans = power(2, onum[n_usize] - 1);
        for i in 1..m {
            ans = mod_norm(ans.wrapping_add(sum[i]));
        }
    } else {
        ans = dp[s[n_usize] as usize];
    }

    print!("{ans}\n");
}
