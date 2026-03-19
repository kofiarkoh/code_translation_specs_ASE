use std::cmp::min;
use std::io::{self, Read};

const INF: i32 = 100_000_000;

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        // C loop condition: for(; scanf("%d%d",&N,&M), N;)
        if n == 0 {
            break;
        }

        // Distance matrices:
        // L: edges of type 'L'
        // S: edges of other type (direct/indirect per spec; C treats non-'L' as S)
        let mut l = vec![vec![INF; n]; n];
        let mut s = vec![vec![INF; n]; n];
        for i in 0..n {
            l[i][i] = 0;
            s[i][i] = 0;
        }

        for _ in 0..m {
            let mut x: usize = it.next().unwrap().parse().unwrap();
            let mut y: usize = it.next().unwrap().parse().unwrap();
            let cost: i32 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap().as_bytes()[0] as char;

            // C uses 1-based input nodes and decrements.
            x -= 1;
            y -= 1;

            if t == 'L' {
                l[x][y] = cost;
                l[y][x] = cost;
            } else {
                s[x][y] = cost;
                s[y][x] = cost;
            }
        }

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut de = vec![0usize; r];
        for i in 0..r {
            let v: usize = it.next().unwrap().parse().unwrap();
            de[i] = v - 1; // C decrements each read node index
        }

        // Floyd–Warshall on both matrices independently.
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    // Use i64 to avoid overflow when adding INF-like values.
                    let cand_s = (s[i][k] as i64) + (s[k][j] as i64);
                    if cand_s < s[i][j] as i64 {
                        s[i][j] = cand_s as i32;
                    }
                    let cand_l = (l[i][k] as i64) + (l[k][j] as i64);
                    if cand_l < l[i][j] as i64 {
                        l[i][j] = cand_l as i32;
                    }
                }
            }
        }

        // DP table: dp[k][i] = min cost after processing k-th visit (0-based),
        // ending at intermediate node i (as in C).
        let mut dp = vec![vec![INF; n]; r];
        dp[0][de[0]] = 0;

        for k in 0..(r.saturating_sub(1)) {
            for i in 0..n {
                for j in 0..n {
                    // dp[k+1][i] = min(dp[k+1][i], dp[k][j] + L[de[k]][j] + S[j][i] + L[i][de[k+1]])
                    let base = dp[k][j] as i64;
                    let cand = base
                        + (l[de[k]][j] as i64)
                        + (s[j][i] as i64)
                        + (l[i][de[k + 1]] as i64);
                    if cand < dp[k + 1][i] as i64 {
                        dp[k + 1][i] = cand as i32;
                    }

                    // Special case in C:
                    // if(i == j) dp[k+1][i] = min(dp[k+1][i], dp[k][j] + L[de[k]][de[k+1]]);
                    if i == j {
                        let cand2 = base + (l[de[k]][de[k + 1]] as i64);
                        if cand2 < dp[k + 1][i] as i64 {
                            dp[k + 1][i] = cand2 as i32;
                        }
                    }
                }
            }
        }

        let mut ans = INF;
        for i in 0..n {
            ans = min(ans, dp[r - 1][i]);
        }
        println!("{ans}");
    }
}
