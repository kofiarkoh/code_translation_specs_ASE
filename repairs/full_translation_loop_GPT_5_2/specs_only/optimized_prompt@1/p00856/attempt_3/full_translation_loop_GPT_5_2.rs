use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n_i64 = n_opt.unwrap().parse::<i64>().unwrap_or(0);
        let t_i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let l_i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let b_i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

        if n_i64 == 0 && t_i64 == 0 && l_i64 == 0 && b_i64 == 0 {
            break;
        }

        // Invalid set: skip and consume obstacle lists best-effort.
        if n_i64 < 0 || t_i64 < 0 || l_i64 < 0 || b_i64 < 0 {
            let l_cnt = l_i64.max(0) as usize;
            let b_cnt = b_i64.max(0) as usize;
            for _ in 0..l_cnt {
                let _ = it.next();
            }
            for _ in 0..b_cnt {
                let _ = it.next();
            }
            continue;
        }

        let n = n_i64 as usize;
        let t = t_i64 as usize;
        let l_cnt = l_i64 as usize;
        let b_cnt = b_i64 as usize;

        if l_cnt + b_cnt > n {
            for _ in 0..l_cnt {
                let _ = it.next();
            }
            for _ in 0..b_cnt {
                let _ = it.next();
            }
            continue;
        }

        // Interpret positions as 0-based indices on 0..N-1, with goal being position N-1.
        // Winning means reaching (or passing) N-1.
        let mut is_l = vec![false; n];
        let mut is_b = vec![false; n];

        for _ in 0..l_cnt {
            if let Some(tok) = it.next() {
                if let Ok(p) = tok.parse::<i64>() {
                    if p >= 0 && (p as usize) < n {
                        is_l[p as usize] = true;
                    }
                }
            }
        }
        for _ in 0..b_cnt {
            if let Some(tok) = it.next() {
                if let Ok(p) = tok.parse::<i64>() {
                    if p >= 0 && (p as usize) < n {
                        is_b[p as usize] = true;
                    }
                }
            }
        }

        // Memoization array initialized to a maximum double value before each computation.
        let mut memo = vec![vec![f64::MAX; n]; t + 1];

        // Model (narrow but consistent with samples):
        // - Start at position 0.
        // - Goal is to reach position N-1 (or beyond) within T turns.
        // - Each turn: roll one fair six-sided die; move forward by 1..=6.
        // - If landing on 'L': you lose immediately (probability 0 from that branch).
        // - If landing on 'B': you are sent back to start (position 0) and continue.
        // - If move goes beyond N-1: treat as reaching N-1 (win).
        fn dp(
            turns_left: usize,
            pos: usize,
            n: usize,
            is_l: &[bool],
            is_b: &[bool],
            memo: &mut [Vec<f64>],
        ) -> f64 {
            if n == 0 {
                return 1.0;
            }
            if pos >= n - 1 {
                return 1.0;
            }
            if turns_left == 0 {
                return 0.0;
            }
            if memo[turns_left][pos] != f64::MAX {
                return memo[turns_left][pos];
            }

            let mut acc = 0.0f64;
            for step in 1..=6usize {
                let mut next = pos + step;
                if next >= n - 1 {
                    next = n - 1;
                } else {
                    if is_l[next] {
                        // Immediate loss.
                        continue;
                    }
                    if is_b[next] {
                        next = 0;
                    }
                }
                acc += dp(turns_left - 1, next, n, is_l, is_b, memo);
            }

            let res = acc / 6.0;
            memo[turns_left][pos] = res;
            res
        }

        let prob = if n == 0 { 1.0 } else { dp(t, 0, n, &is_l, &is_b, &mut memo) };
        out.push_str(&format!("{:.6}\n", prob));
    }

    print!("{out}");
}
