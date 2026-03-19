use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0i32; w]; h];
    for i in 0..h {
        for j in 0..w {
            a[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // DP over time steps t = r+c (0-based). Both traversals take exactly one move per step,
    // so they share the same t at all times.
    //
    // State: dp[r1][r2] at time t, where:
    //   c1 = t - r1, c2 = t - r2
    // and both (r1,c1), (r2,c2) are within bounds.
    //
    // Transition from time t-1 by moving each traversal either down (r+1) or right (r same).
    // Since t increases by 1, "right" corresponds to keeping r the same, "down" corresponds to r+1.
    //
    // Value added at time t is a[r1][c1] + a[r2][c2], but if same cell then count once.
    const NEG_INF: i32 = i32::MIN / 4;

    let mut dp_prev = vec![vec![NEG_INF; h]; h];
    let mut dp_cur = vec![vec![NEG_INF; h]; h];

    // t = 0: both at (0,0)
    dp_prev[0][0] = a[0][0];

    let t_max = (h - 1) + (w - 1);
    for t in 1..=t_max {
        for r1 in 0..h {
            for r2 in 0..h {
                dp_cur[r1][r2] = NEG_INF;
            }
        }

        // Valid r range for a given t: r in [max(0, t-(w-1)) .. min(h-1, t)]
        let r_min = if t >= (w - 1) { t - (w - 1) } else { 0 };
        let r_max = std::cmp::min(h - 1, t);

        for r1 in r_min..=r_max {
            let c1 = t - r1;
            if c1 >= w {
                continue;
            }
            for r2 in r_min..=r_max {
                let c2 = t - r2;
                if c2 >= w {
                    continue;
                }

                let mut best = NEG_INF;

                // Previous positions at time t-1:
                // If current move was right, previous r is same; if down, previous r is r-1.
                // Enumerate four combinations.
                for dr1 in 0..=1 {
                    for dr2 in 0..=1 {
                        // dr=0 means came from right (r unchanged), dr=1 means came from down (r-1).
                        if dr1 == 1 && r1 == 0 {
                            continue;
                        }
                        if dr2 == 1 && r2 == 0 {
                            continue;
                        }
                        let pr1 = r1 - dr1;
                        let pr2 = r2 - dr2;

                        // At time t-1, columns are (t-1 - pr)
                        let pc1 = (t - 1) - pr1;
                        let pc2 = (t - 1) - pr2;
                        if pc1 >= w || pc2 >= w {
                            continue;
                        }

                        best = max(best, dp_prev[pr1][pr2]);
                    }
                }

                if best == NEG_INF {
                    continue;
                }

                let mut add = a[r1][c1] + a[r2][c2];
                if r1 == r2 && c1 == c2 {
                    add = a[r1][c1];
                }

                dp_cur[r1][r2] = best + add;
            }
        }

        std::mem::swap(&mut dp_prev, &mut dp_cur);
    }

    let ans = dp_prev[h - 1][h - 1];
    println!("{}", ans);
}
