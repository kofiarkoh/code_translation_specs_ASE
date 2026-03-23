use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Taxi {
    fare: i32,
    limit: usize,
}

fn main() {
    // Read entire stdin; if parsing fails, exit with nonzero status and no output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut taxi = vec![
        Taxi {
            fare: 0,
            limit: 0
        };
        n
    ];
    for i in 0..n {
        let fare: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        let limit_i32: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        // Spec assumes limits are within 0..=5000; if not, behavior is undefined.
        taxi[i] = Taxi {
            fare,
            limit: limit_i32 as usize,
        };
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..k {
        let mut a: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        let mut b: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        // Input is 1-based; convert to 0-based.
        a -= 1;
        b -= 1;
        let au = a as usize;
        let bu = b as usize;
        adj[au].push(bu);
        adj[bu].push(au);
    }

    // State space: (town, remaining allowance), remaining in 0..=5000.
    const MAX_R: usize = 5000;

    // best[t][r] = Some(min_cost) else None (unreached).
    let mut best: Vec<Vec<Option<i32>>> = vec![vec![None; MAX_R + 1]; n];

    // Dijkstra over expanded state graph.
    // Heap items: (cost, town, remaining). Use Reverse for min-heap behavior.
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    // Initial state: town 0, remaining 0, cost 0.
    best[0][0] = Some(0);
    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((cost, town, rem))) = heap.pop() {
        // Skip stale entries.
        if best[town][rem] != Some(cost) {
            continue;
        }

        // Destination condition: town == N-1 regardless of remaining.
        if town + 1 == n {
            print!("{cost}\n");
            return;
        }

        // Free road traversal if rem > 0: to (neighbor, rem-1) with same cost.
        if rem > 0 {
            let next_rem = rem - 1;
            for &to in &adj[town] {
                if best[to][next_rem].map_or(true, |c| cost < c) {
                    best[to][next_rem] = Some(cost);
                    heap.push(Reverse((cost, to, next_rem)));
                }
            }
        }

        // Purchase taxi in current town: to (town, limit) with cost + fare.
        let next_rem = taxi[town].limit;
        // Spec assumes next_rem within 0..=5000; otherwise undefined.
        if next_rem <= MAX_R {
            let next_cost = cost + taxi[town].fare;
            if best[town][next_rem].map_or(true, |c| next_cost < c) {
                best[town][next_rem] = Some(next_cost);
                heap.push(Reverse((next_cost, town, next_rem)));
            }
        }
    }

    // Unreachable: terminate normally without printing anything.
}
