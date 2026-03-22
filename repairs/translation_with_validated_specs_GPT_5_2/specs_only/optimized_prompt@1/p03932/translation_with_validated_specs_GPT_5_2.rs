use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input: unspecified; do nothing.
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0i64; w]; h];
    for i in 0..h {
        for j in 0..w {
            grid[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // DP over synchronized steps k = 0..(h+w-2).
    // State: dp[i1][i2] = best total after k steps where
    // agent1 at (i1, j1=k-i1), agent2 at (i2, j2=k-i2).
    //
    // Important semantic note from spec:
    // - No explicit unreachable marker is available in the intended implementation;
    //   unreachable/unused states are indistinguishable from valid zero-valued states.
    // - This is only correct if all cell values are non-negative.
    //
    // We therefore initialize dp with 0 and only compute transitions for in-bounds
    // (i, j) pairs; out-of-bounds are skipped.
    let mut dp = vec![vec![0i64; h]; h];

    let steps = h + w - 2;
    for k in 0..=steps {
        let mut next = vec![vec![0i64; h]; h];

        for i1 in 0..h {
            let j1 = match k.checked_sub(i1) {
                Some(v) => v,
                None => continue,
            };
            if j1 >= w {
                continue;
            }

            for i2 in 0..h {
                let j2 = match k.checked_sub(i2) {
                    Some(v) => v,
                    None => continue,
                };
                if j2 >= w {
                    continue;
                }

                // Best previous among the four combinations of moves:
                // (down/down), (down/right), (right/down), (right/right)
                // where "down" means i increased by 1, so previous i is i-1;
                // "right" means j increased by 1, so previous i unchanged.
                let mut best_prev = dp[i1][i2];

                if i1 > 0 {
                    best_prev = best_prev.max(dp[i1 - 1][i2]);
                }
                if i2 > 0 {
                    best_prev = best_prev.max(dp[i1][i2 - 1]);
                }
                if i1 > 0 && i2 > 0 {
                    best_prev = best_prev.max(dp[i1 - 1][i2 - 1]);
                }

                // Add current cell values, counting once if both agents coincide.
                let mut add = grid[i1][j1];
                if i1 != i2 || j1 != j2 {
                    add += grid[i2][j2];
                }

                next[i1][i2] = best_prev + add;
            }
        }

        dp = next;
    }

    // Final synchronized state: both at bottom-right.
    let ans = dp[h - 1][h - 1];
    print!("{ans}");
}
