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

    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = vec![0i32; n];
        for i in 0..n {
            a[i] = it.next().unwrap().parse().unwrap();
        }

        // dp[i][k] corresponds to C's dp[i][k], initialized to 0.
        // Dimensions: n x n (C uses fixed 102).
        let mut dp = vec![vec![0i32; n]; n];

        // Preserve the exact loop structure/semantics from C.
        for k in 1..n {
            for i in 0..n {
                for j in 0..k {
                    let mut x = i + j + 1;
                    if x >= n {
                        x -= n; // only one subtraction, as in C
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

        let mut ans = 0i32;
        for i in 0..n {
            if dp[i][n - 1] > ans {
                ans = dp[i][n - 1];
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
