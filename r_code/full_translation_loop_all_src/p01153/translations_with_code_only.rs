use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Match C fixed-size arrays (102), but we only use first n elements.
    let mut a = [0i32; 102];
    let mut dp = [[0i32; 102]; 102];

    let mut out = String::new();

    for _case in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        for i in 0..n {
            a[i] = it.next().unwrap().parse().unwrap();
        }

        // memset(dp, 0, sizeof(dp));
        for i in 0..102 {
            for k in 0..102 {
                dp[i][k] = 0;
            }
        }

        for k in 1..n {
            for i in 0..n {
                for j in 0..k {
                    let mut x = i + j + 1;
                    if x >= n {
                        x -= n;
                    }
                    let mut v = a[x] - a[i];
                    if v < 0 {
                        v = -v;
                    }
                    v += dp[x][k - j - 1] + dp[i][j];
                    if v > dp[i][k] {
                        dp[i][k] = v;
                    }
                }
            }
        }

        let mut ans: i32 = 0;
        for i in 0..n {
            if dp[i][n - 1] > ans {
                ans = dp[i][n - 1];
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
