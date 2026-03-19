use std::cmp::min;
use std::io::{self, Read};

fn clamp_u8_from_i32(x: i32) -> usize {
    if x < 0 {
        0
    } else if x > 255 {
        255
    } else {
        x as usize
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let inf: i64 = i64::MAX / 4;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let _m: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        if n == 0 {
            break;
        }

        // Read increments
        let mut incs: Vec<i32> = Vec::with_capacity(_m);
        for _ in 0.._m {
            let a: i32 = it.next().unwrap().parse().unwrap();
            incs.push(a);
        }

        // Read targets
        let mut targets: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            let t: i32 = it.next().unwrap().parse().unwrap();
            targets.push(t);
        }

        // DP over 256 clamped states.
        // dp[s] = minimum cost to be at state s after processing current number of targets.
        let mut dp = vec![inf; 256];
        dp[128] = 0;

        for &t in &targets {
            let mut next = vec![inf; 256];

            for s in 0..256 {
                let base = dp[s];
                if base == inf {
                    continue;
                }
                let s_i32 = s as i32;

                for &a in &incs {
                    let sp = clamp_u8_from_i32(s_i32 + a);
                    let diff = (t - sp as i32) as i64;
                    let cost = diff * diff;
                    let cand = base + cost;
                    if cand < next[sp] {
                        next[sp] = cand;
                    }
                }
            }

            dp = next;
        }

        let mut ans = inf;
        for &v in &dp {
            ans = min(ans, v);
        }

        println!("{}", ans);
    }
}
