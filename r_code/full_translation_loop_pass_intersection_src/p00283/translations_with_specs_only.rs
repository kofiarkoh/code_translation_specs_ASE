use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy)]
struct Action {
    reduction: i32, // positive per precondition
    gain: i32,      // nonnegative per precondition
    req: i32,       // nonnegative per precondition
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let t0 = match it.next() {
            Some(s) => s.parse::<i32>().unwrap(),
            None => break,
        };
        let n = match it.next() {
            Some(s) => s.parse::<usize>().unwrap(),
            None => break,
        };

        if t0 == 0 && n == 0 {
            break;
        }

        // If the dataset is incomplete/malformed while reading action definitions,
        // terminate immediately with nonzero exit status and no further outputs.
        let mut actions: Vec<Action> = Vec::with_capacity(n);
        for _ in 0..n {
            let r = match it.next() {
                Some(s) => s.parse::<i32>().unwrap(),
                None => process::exit(1),
            };
            let g = match it.next() {
                Some(s) => s.parse::<i32>().unwrap(),
                None => process::exit(1),
            };
            let q = match it.next() {
                Some(s) => s.parse::<i32>().unwrap(),
                None => process::exit(1),
            };
            actions.push(Action {
                reduction: r,
                gain: g,
                req: q,
            });
        }

        // Bounded-state DP over T in [0..100], K in [0..100].
        // Start resource level is not specified; narrowest reasonable interpretation is 0.
        // Success is reaching any state with T <= 0, which corresponds to T == 0 in the bounded DP.
        const MAX_T: usize = 100;
        const MAX_K: usize = 100;
        const INF: i32 = i32::MAX / 4;

        // Preconditions say target is within 0..=100; behavior outside is undefined.
        // We do not add extra handling beyond what Rust naturally does.
        let t0u = t0 as usize;

        let mut dist = vec![vec![INF; MAX_K + 1]; MAX_T + 1];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        dist[t0u][0] = 0;
        q.push_back((t0u, 0));

        while let Some((t, k)) = q.pop_front() {
            let d = dist[t][k];
            for a in &actions {
                if a.req <= k as i32 {
                    let next_t_i = t as i32 - a.reduction;
                    let next_t = if next_t_i <= 0 {
                        0usize
                    } else {
                        // State space is defined only up to 100; precondition says within range.
                        next_t_i as usize
                    };
                    let next_k_i = k as i32 + a.gain;
                    let next_k = min(MAX_K, if next_k_i < 0 { 0 } else { next_k_i as usize });

                    if dist[next_t][next_k] > d + 1 {
                        dist[next_t][next_k] = d + 1;
                        q.push_back((next_t, next_k));
                    }
                }
            }
        }

        let best = dist[0].iter().copied().min().unwrap_or(INF);
        if best >= INF {
            outputs.push("no solution".to_string());
        } else {
            outputs.push(best.to_string());
        }
    }

    print!("{}", outputs.join("\n"));
}
