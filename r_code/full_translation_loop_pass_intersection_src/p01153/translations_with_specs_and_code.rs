use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = vec![0i32; n];
        for i in 0..n {
            a[i] = it.next().unwrap().parse().unwrap();
        }

        if n <= 1 {
            out.push_str("0\n");
            continue;
        }

        // dp[i][k] = maximum score for a circularly contiguous segment
        // starting at i with length (k+1). (k is length-1)
        let mut dp = vec![vec![0i32; n]; n];

        // Build by increasing segment length (k+1), where k from 1..n-1
        for k in 1..n {
            for i in 0..n {
                let mut best = 0i32;
                for j in 0..k {
                    // Split: left length = j+1, right length = k-j
                    // Right part starts at x = (i + (j+1)) mod n
                    let mut x = i + j + 1;
                    if x >= n {
                        x -= n;
                    }

                    let mut v = a[x] - a[i];
                    if v < 0 {
                        v = -v;
                    }
                    // dp[x][k-j-1] corresponds to right length (k-j) => (k-j-1)
                    // dp[i][j] corresponds to left length (j+1) => j
                    v += dp[x][k - j - 1] + dp[i][j];

                    if v > best {
                        best = v;
                    }
                }
                dp[i][k] = best;
            }
        }

        let mut ans = 0i32;
        for i in 0..n {
            ans = ans.max(dp[i][n - 1]);
        }
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
