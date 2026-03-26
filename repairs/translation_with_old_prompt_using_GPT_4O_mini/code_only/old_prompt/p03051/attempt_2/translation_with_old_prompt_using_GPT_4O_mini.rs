use std::io::{self, BufRead};

const P: i64 = 1_000_000_007;

fn mod_value(a: i64) -> i64 {
    let mut a = a % P;
    if a < 0 {
        a += P;
    }
    a
}

fn power(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        power(mod_value(a * a), n / 2)
    } else {
        mod_value(a * power(a, n - 1))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let m: usize = 1 << 20;
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut a = vec![0; n + 1];
    let mut s = vec![0; n + 1];
    let mut onum = vec![0; n + 1];

    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
        s[i] = s[i - 1] ^ a[i];
        onum[i] = onum[i - 1] + if s[i] == 0 { 1 } else { 0 };
    }

    let mut prev = vec![0; m];
    let mut dp = vec![0; m];
    let mut sum = vec![0; m];
    let mut osum = vec![1; m];

    for i in 1..=n {
        let x = s[i] as usize;
        dp[x] = mod_value(sum[x] * (onum[i] - onum[prev[x]]) + osum[x]);
        osum[x] = mod_value(osum[x] + sum[x] * (onum[i] - onum[prev[x]]));
        sum[x] = mod_value(sum[x] + dp[x]);
        prev[x] = i;
    }

    let mut ans = 0;
    if s[n] == 0 {
        ans = power(2, onum[n] - 1);
        for i in 1..m {
            ans = mod_value(ans + sum[i]);
        }
    } else {
        ans = dp[s[n] as usize];
    }

    println!("{}", ans);
}
