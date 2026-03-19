use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read all input; if parsing fails, exit with nonzero status and no guaranteed output.
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

    // Taxi fare and limit per town.
    let mut fare: Vec<i32> = vec![0; n];
    let mut limit: Vec<usize> = vec![0; n];
    for i in 0..n {
        fare[i] = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        limit[i] = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
    }

    // Undirected adjacency list.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..k {
        let mut a: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        let mut b: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        // Input is 1-based in the C code.
        a -= 1;
        b -= 1;
        adj[a].push(b);
        adj[b].push(a);
    }

    // Dijkstra over states (town, remaining ride allowance).
    // The C code uses fixed arrays [5000][5001]; spec guarantees bounds.
    // We allocate based on actual n and max_limit (<= 5000 by precondition).
    let max_limit = limit.iter().copied().max().unwrap_or(0);
    let rmax = max_limit;

    // dist[t][r] = best known cost, None if unknown.
    let mut dist: Vec<Vec<Option<i32>>> = vec![vec![None; rmax + 1]; n];

    let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    // initial state (0, 0) with cost 0
    dist[0][0] = Some(0);
    heap.push((Reverse(0), 0, 0));

    while let Some((Reverse(cur_cost), town, r)) = heap.pop() {
        // Skip stale entries.
        if dist[town][r] != Some(cur_cost) {
            continue;
        }

        if town == n - 1 {
            // Print upon first reaching destination with minimal cost.
            print!("{cur_cost}\n");
            return;
        }

        // Move to neighbors if remaining allowance > 0.
        if r > 0 {
            let nr = r - 1;
            for &u in &adj[town] {
                if dist[u][nr].map_or(true, |old| cur_cost < old) {
                    dist[u][nr] = Some(cur_cost);
                    heap.push((Reverse(cur_cost), u, nr));
                }
            }
        }

        // Pay taxi fare at current town to reset allowance to limit[town].
        let nr = limit[town];
        if nr <= rmax {
            let next_cost = cur_cost + fare[town];
            if dist[town][nr].map_or(true, |old| next_cost < old) {
                dist[town][nr] = Some(next_cost);
                heap.push((Reverse(next_cost), town, nr));
            }
        }
    }

    // If unreachable: per spec, no output is guaranteed; terminate normally without printing.
}
