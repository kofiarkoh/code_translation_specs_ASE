use std::cmp::min;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Opt {
    a: i32,
    b: i32,
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Read stages and their options.
        let mut stages: Vec<Vec<Opt>> = Vec::with_capacity(n);
        for _ in 0..n {
            let k: usize = it.next().unwrap().parse().unwrap();
            let mut opts: Vec<Opt> = Vec::with_capacity(k);
            for _ in 0..k {
                let a: i32 = it.next().unwrap().parse().unwrap();
                let b: i32 = it.next().unwrap().parse().unwrap();
                opts.push(Opt { a, b });
            }
            stages.push(opts);
        }

        // DP: dp[s][j][o] = min cost to reach option o at stage s using exactly j skips.
        // Use i64 internally to avoid accidental overflow in intermediate computations,
        // but the spec assumes 32-bit signed fits; final output is printed as integer.
        let inf: i64 = i64::MAX / 4;

        let mut dp: Vec<Vec<Vec<i64>>> = Vec::with_capacity(n);
        for s in 0..n {
            let k = stages[s].len();
            let mut by_skips: Vec<Vec<i64>> = Vec::with_capacity(m + 1);
            for _ in 0..=m {
                by_skips.push(vec![inf; k]);
            }
            dp.push(by_skips);
        }

        // Initialization: start at stage 0 with any option, cost 0, 0 skips.
        for o in 0..stages[0].len() {
            dp[0][0][o] = 0;
        }

        // Transitions.
        for s in 0..n {
            for j in 0..=m {
                let cur_len = stages[s].len();
                for o1 in 0..cur_len {
                    let base = dp[s][j][o1];
                    if base == inf {
                        continue;
                    }
                    let from = stages[s][o1];

                    // Move to s+1 without consuming a skip.
                    if s + 1 < n {
                        for (o2, &to) in stages[s + 1].iter().enumerate() {
                            let cost = (from.b as i64 + to.b as i64)
                                * (from.a as i64 - to.a as i64).abs();
                            let cand = base + cost;
                            if cand < dp[s + 1][j][o2] {
                                dp[s + 1][j][o2] = cand;
                            }
                        }
                    }

                    // Move to s+2 consuming one skip.
                    if s + 2 < n && j + 1 <= m {
                        for (o2, &to) in stages[s + 2].iter().enumerate() {
                            let cost = (from.b as i64 + to.b as i64)
                                * (from.a as i64 - to.a as i64).abs();
                            let cand = base + cost;
                            if cand < dp[s + 2][j + 1][o2] {
                                dp[s + 2][j + 1][o2] = cand;
                            }
                        }
                    }
                }
            }
        }

        // Result:
        // - Minimum cost ending at stage n-1 with any skips <= m.
        // - Or ending at stage n-2 with strictly fewer than m skips.
        let mut ans = inf;

        // End at n-1 with j in [0..=m]
        let last = n - 1;
        for j in 0..=m {
            for &v in &dp[last][j] {
                ans = min(ans, v);
            }
        }

        // End at n-2 with j in [0..m-1] (strictly fewer than m skips)
        if n >= 2 && m >= 1 {
            let second_last = n - 2;
            for j in 0..m {
                for &v in &dp[second_last][j] {
                    ans = min(ans, v);
                }
            }
        }

        // The spec does not define behavior if no valid path exists; we print the computed minimum.
        outputs.push(format!("{}", ans));
    }

    print!("{}", outputs.join("\n"));
}
