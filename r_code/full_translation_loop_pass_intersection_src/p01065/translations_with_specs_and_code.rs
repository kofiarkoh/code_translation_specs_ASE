use std::io::{self, Read};

const BASE: usize = 300;
const DBASE: usize = 600;
const NEG: i32 = -1;

fn main() {
    // Read all input as bytes; input is specified to be non-negative integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let x: usize = it.next().unwrap().parse().unwrap();
    let y: usize = it.next().unwrap().parse().unwrap();

    // dp[i][a][b_shift] = max value, or -1 if unreachable.
    // b_shift represents actual B as (b_shift - BASE), so b_shift in 0..=DBASE.
    let mut dp = vec![vec![vec![NEG; DBASE + 2]; BASE + 2]; n + 1];
    dp[0][0][BASE] = 0;

    for i in 0..n {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: i32 = it.next().unwrap().parse().unwrap();

        for j in 0..=BASE {
            for k in 0..=DBASE {
                let cur = dp[i][j][k];
                if cur < 0 {
                    continue;
                }

                // Ignore offer
                if dp[i + 1][j][k] < cur {
                    dp[i + 1][j][k] = cur;
                }

                // Mode 1: (A + a, B - b), value unchanged, only if A bound satisfied.
                // Note: The C code does not bounds-check k-b; preconditions guarantee it stays in range.
                if x >= j + a {
                    let nk = k.wrapping_sub(b);
                    if nk <= DBASE && dp[i + 1][j + a][nk] < cur {
                        dp[i + 1][j + a][nk] = cur;
                    }
                }

                // Mode 2: (A, B + c), value + d
                // Note: The C code does not bounds-check k+c; preconditions guarantee it stays in range.
                let nk2 = k + c;
                if nk2 <= DBASE && dp[i + 1][j][nk2] < cur + d {
                    dp[i + 1][j][nk2] = cur + d;
                }
            }
        }
    }

    // Final answer: max dp[n][j][k] with j <= x and actual B <= y,
    // i.e., k <= y + BASE.
    let mut ans: i32 = 0;
    let b_limit_shift = y + BASE;
    for j in 0..=x.min(BASE) {
        for k in 0..=DBASE {
            let v = dp[n][j][k];
            if v > ans && b_limit_shift >= k {
                ans = v;
            }
        }
    }

    print!("{ans}\n");
}
