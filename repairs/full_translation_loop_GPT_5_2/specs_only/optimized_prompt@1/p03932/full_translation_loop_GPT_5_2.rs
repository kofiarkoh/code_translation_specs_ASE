use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No input: unspecified; exit quietly.
    };
    let h: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Read grid values (non-negative per spec; we store as i64 for DP arithmetic).
    let mut grid = vec![vec![0i64; w]; h];
    for y in 0..h {
        for x in 0..w {
            if let Some(s) = it.next() {
                grid[y][x] = s.parse::<i64>().unwrap();
            } else {
                // Input format incorrect: unspecified; keep default 0s.
                // This matches "may not function as intended" without adding explicit errors.
                grid[y][x] = 0;
            }
        }
    }

    // Two entities move from (0,0) to (w-1,h-1) with steps Right/Down.
    // Standard DP over "time" t = x+y, tracking both x positions (y derived).
    // If both entities are on the same cell at a step, count that cell only once.
    //
    // dp[x1][x2] = max sum up to current t with entity1 at (x1, t-x1) and entity2 at (x2, t-x2).
    // Transition from previous t-1 by moving each entity either Right (x+1) or Down (x unchanged).
    //
    // Corner cases:
    // - If w==0 or h==0, behavior is not defined by spec; we output 0 as narrowest safe behavior.
    if w == 0 || h == 0 {
        print!("0\n");
        return;
    }

    let neg_inf: i64 = i64::MIN / 4;
    let mut dp = vec![vec![neg_inf; w]; w];
    dp[0][0] = grid[0][0];

    let max_t = (w - 1) + (h - 1);
    for t in 1..=max_t {
        let mut ndp = vec![vec![neg_inf; w]; w];

        // x ranges where y=t-x is within [0, h-1]
        let x_min = if t >= (h - 1) { t - (h - 1) } else { 0 };
        let x_max = if t <= (w - 1) { t } else { w - 1 };

        for x1 in x_min..=x_max {
            let y1 = t - x1;
            if y1 >= h {
                continue;
            }
            for x2 in x_min..=x_max {
                let y2 = t - x2;
                if y2 >= h {
                    continue;
                }

                let mut best_prev = neg_inf;

                // Previous positions at t-1:
                // For each entity: came from Left (x-1, same y) => moved Right, or from Up (x, y-1) => moved Down.
                // In terms of x: if moved Right, previous x = x-1; if moved Down, previous x = x.
                for (px1, ok1) in [
                    (x1, y1 > 0),          // moved Down into (x1,y1)
                    (x1.wrapping_sub(1), x1 > 0), // moved Right into (x1,y1)
                ] {
                    if !ok1 {
                        continue;
                    }
                    for (px2, ok2) in [
                        (x2, y2 > 0),
                        (x2.wrapping_sub(1), x2 > 0),
                    ] {
                        if !ok2 {
                            continue;
                        }
                        let prev = dp[px1][px2];
                        if prev > best_prev {
                            best_prev = prev;
                        }
                    }
                }

                if best_prev == neg_inf {
                    continue;
                }

                let mut add = grid[y1][x1];
                if x1 != x2 || y1 != y2 {
                    add += grid[y2][x2];
                }

                let cand = best_prev.saturating_add(add);
                if cand > ndp[x1][x2] {
                    ndp[x1][x2] = cand;
                }
            }
        }

        dp = ndp;
    }

    let ans = dp[w - 1][w - 1];
    // If unreachable (shouldn't happen for w,h>=1), print 0 as narrowest defined output.
    if ans == neg_inf {
        print!("0\n");
    } else {
        print!("{ans}\n");
    }
}
