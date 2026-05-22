use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::process;

const MAX_R: usize = 5000;

fn main() {
    // Read entire stdin; if any required value is missing, exit nonzero with no output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    let mut fare: Vec<i32> = Vec::with_capacity(n);
    let mut limit: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let f: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let l: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        fare.push(f);
        limit.push(l);
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..k {
        let a1: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b1: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        // Specification: endpoints are 1-based and converted to 0-based by subtracting 1.
        // If inputs violate assumptions, behavior is undefined; we do not add extra checks.
        let a = a1 - 1;
        let b = b1 - 1;
        adj[a].push(b);
        adj[b].push(a);
    }

    // Dijkstra over state space (town, remaining allowance).
    // dist[t][r] = best known total fare to reach state.
    let inf: i32 = i32::MAX;
    let mut dist: Vec<Vec<i32>> = vec![vec![inf; MAX_R + 1]; n];

    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    dist[0][0] = 0;
    pq.push((Reverse(0), 0, 0));

    while let Some((Reverse(cost), t, r)) = pq.pop() {
        if cost != dist[t][r] {
            continue;
        }

        // Terminate as soon as we extract any destination state at minimum cost.
        if t + 1 == n {
            print!("{}\n", cost);
            return;
        }

        // Purchase taxi in current town: allowed regardless of r, may be repeated.
        // Cost arithmetic is specified to fit in signed int; use i32 and wrapping add
        // to emulate C-like int behavior if overflow were to occur (though assumed not).
        let new_r = limit[t];
        if new_r <= MAX_R {
            let new_cost = cost.wrapping_add(fare[t]);
            if new_cost < dist[t][new_r] {
                dist[t][new_r] = new_cost;
                pq.push((Reverse(new_cost), t, new_r));
            }
        }

        // Free road traversal if r > 0.
        if r > 0 {
            let nr = r - 1;
            for &to in &adj[t] {
                if cost < dist[to][nr] {
                    dist[to][nr] = cost;
                    pq.push((Reverse(cost), to, nr));
                }
            }
        }
    }

    // Unreachable: exit normally without printing.
}
