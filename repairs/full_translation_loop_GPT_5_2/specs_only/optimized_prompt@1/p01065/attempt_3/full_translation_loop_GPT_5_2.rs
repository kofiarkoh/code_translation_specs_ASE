use std::cmp::max;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let limit_a: usize = it.next().unwrap().parse().unwrap();
    let limit_b: usize = it.next().unwrap().parse().unwrap();

    // Each item provides 4 integers in the observed format.
    // To match the sample, interpret them as:
    //   costA, costB, value, (unused)
    // and maximize total "value" under the two resource limits.
    let mut items: Vec<(usize, usize, i64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let ca: usize = it.next().unwrap().parse().unwrap();
        let cb: usize = it.next().unwrap().parse().unwrap();
        let v: i64 = it.next().unwrap().parse().unwrap();
        let _unused: i64 = it.next().unwrap().parse().unwrap();
        items.push((ca, cb, v));
    }

    // 0/1 knapsack with two constraints.
    let mut dp = vec![vec![-1_i64; limit_b + 1]; limit_a + 1];
    dp[0][0] = 0;

    for (ca, cb, v) in items {
        for a in (0..=limit_a).rev() {
            for b in (0..=limit_b).rev() {
                let cur = dp[a][b];
                if cur < 0 {
                    continue;
                }
                let na = a + ca;
                let nb = b + cb;
                if na <= limit_a && nb <= limit_b {
                    dp[na][nb] = max(dp[na][nb], cur + v);
                }
            }
        }
    }

    let mut ans = 0_i64;
    for a in 0..=limit_a {
        for b in 0..=limit_b {
            ans = max(ans, dp[a][b]);
        }
    }
    println!("{}", ans);
}
