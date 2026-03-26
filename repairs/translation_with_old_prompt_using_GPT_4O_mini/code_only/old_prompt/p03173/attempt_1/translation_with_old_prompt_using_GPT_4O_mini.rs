use std::io::{self, BufRead};

const SUP: i64 = 1 << 60; // Equivalent to (long long)1 << 60
const MAX_N: usize = 401;

fn recursion(l: usize, r: usize, dp: &mut [[i64; MAX_N]], sum: &[i64]) -> i64 {
    if dp[l][r] != 0 || l == r {
        return dp[l][r];
    } else {
        dp[l][r] = SUP;
    }

    for k in l + 1..=r {
        let tmp = recursion(l, k - 1, dp, sum) + recursion(k, r, dp, sum);
        if tmp < dp[l][r] {
            dp[l][r] = tmp;
        }
    }
    dp[l][r] += sum[r] - sum[l - 1];
    dp[l][r]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a = vec![0; MAX_N];
    let mut sum = vec![0; MAX_N];
    let mut dp = [[0; MAX_N]; MAX_N];

    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    for i in 1 {
        sum[0] = 0;
        for i in 1..=n {
            sum[i] = sum[i - 1] + a[i];
        }
    }

    println!("{}", recursion(1, n, &mut dp, &sum));
}
