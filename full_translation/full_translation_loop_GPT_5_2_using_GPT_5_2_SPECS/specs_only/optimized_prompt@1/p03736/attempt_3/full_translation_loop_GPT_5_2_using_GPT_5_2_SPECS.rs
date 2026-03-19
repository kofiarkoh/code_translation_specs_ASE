use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = i64::MAX / 4;

fn abs_i64(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1; // current (0-based)
    let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1; // target (0-based)

    // dp[i] is the per-index adjustment value; seeded at target position.
    let mut dp = vec![INF; n];
    dp[b] = 0;

    let mut movement: i64 = 0;

    for _ in 0..m {
        let x: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;

        movement += abs_i64(x as i64 - a as i64);

        // Update rule consistent with:
        // - adjustment depends on initial target and visited positions
        // - per-index values only improve by minima
        // - uses previous current position (a) and new current position (x)
        //
        // We interpret dp[i] as the best extra adjustment when the "saved" position is i.
        // When moving a -> x, we can:
        //   1) keep saved i: dp'[i] = min(dp'[i], dp[i])
        //   2) switch saved to previous a: dp'[a] = min(dp'[a], dp[i] + |i - x|)
        //
        // Notably, we do NOT add |a-x| into dp; that distance is already accounted in `movement`.
        let mut best_for_a = INF;
        for i in 0..n {
            let v = dp[i];
            if v >= INF / 2 {
                continue;
            }
            let cand = v + abs_i64(i as i64 - x as i64);
            if cand < best_for_a {
                best_for_a = cand;
            }
        }
        dp[a] = min(dp[a], best_for_a);

        a = x;
    }

    let best_adjust = dp.into_iter().min().unwrap_or(INF);
    let ans = movement + best_adjust;
    print!("{}", ans);
}
