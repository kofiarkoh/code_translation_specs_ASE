use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();

        // C arrays are size 102; we allocate dynamically but keep same indexing behavior.
        let mut a = vec![0i32; n];
        for i in 0..n {
            a[i] = it.next().unwrap().parse().unwrap();
        }

        // dp[i][k] as in C: int dp[102][102]; initialized to 0.
        let mut dp = vec![vec![0i32; n.max(1)]; n.max(1)];

        for k in 1..n {
            for i in 0..n {
                for j in 0..k {
                    let mut x = i + j + 1;
                    if x >= n {
                        x -= n; // matches the single subtraction in C
                    }

                    let mut v = a[x] - a[i];
                    if v < 0 {
                        v = -v; // abs for i32, matching C's logic
                    }
                    v += dp[x][k - j - 1] + dp[i][j];

                    if v > dp[i][k] {
                        dp[i][k] = v;
                    }
                }
            }
        }

        let mut ans: i32 = 0;
        if n > 0 {
            for i in 0..n {
                if dp[i][n - 1] > ans {
                    ans = dp[i][n - 1];
                }
            }
        }
        out.push_str(&format!("{}\n", ans));
    }

    print!("{}", out);
}
