use std::io::{self, Read};

const BASE: usize = 300;
const DBASE: usize = 600;

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Per spec, inputs are non-negative; behavior for invalid input is undefined.
    let n: usize = it.next().unwrap().parse().unwrap();
    let x: usize = it.next().unwrap().parse().unwrap();
    let y: usize = it.next().unwrap().parse().unwrap();

    // dp[i][j][k] in C, initialized to -1, with dp[0][0][BASE] = 0.
    // We'll keep only two layers (i and i+1) to be memory-friendly and safe.
    let mut cur = vec![vec![-1i32; DBASE + 2]; BASE + 2];
    let mut nxt = vec![vec![-1i32; DBASE + 2]; BASE + 2];
    cur[0][BASE] = 0;

    for _ in 0..n {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: i32 = it.next().unwrap().parse().unwrap();

        // reset nxt to -1
        for row in nxt.iter_mut() {
            row.fill(-1);
        }

        for j in 0..=BASE {
            for k in 0..=DBASE {
                let v = cur[j][k];
                if v < 0 {
                    continue;
                }

                // Not take / carry over
                if nxt[j][k] < v {
                    nxt[j][k] = v;
                }

                // Transition 1: if (x >= j + a) dp[i+1][j+a][k-b] = dp[i][j][k]
                // Note: C code does not bounds-check k-b; it relies on valid inputs.
                // We preserve semantics under the spec's "valid input" assumption.
                if j + a <= x && j + a <= BASE && k >= b {
                    let nk = k - b;
                    if nxt[j + a][nk] < v {
                        nxt[j + a][nk] = v;
                    }
                }

                // Transition 2: dp[i+1][j][k+c] = dp[i][j][k] + d
                if k + c <= DBASE {
                    let nv = v + d;
                    if nxt[j][k + c] < nv {
                        nxt[j][k + c] = nv;
                    }
                }
            }
        }

        std::mem::swap(&mut cur, &mut nxt);
    }

    // Final answer:
    // for j=0..x, k=0..600: if (ans < dp[n][j][k] && y+BASE >= k) ans = dp[n][j][k]
    let mut ans: i32 = 0;
    let y_limit = y + BASE;
    let j_max = x.min(BASE);
    for j in 0..=j_max {
        for k in 0..=DBASE {
            let v = cur[j][k];
            if v > ans && y_limit >= k {
                ans = v;
            }
        }
    }

    print!("{ans}\n");
}
