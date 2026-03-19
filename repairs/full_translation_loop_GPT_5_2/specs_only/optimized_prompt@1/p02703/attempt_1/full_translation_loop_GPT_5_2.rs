use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cost: i64,
    time: i64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    if n_opt.is_none() {
        return;
    }
    let n: usize = n_opt.unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let budget: i64 = it.next().unwrap().parse().unwrap();

    // Special-case behavior is mentioned but not specified (no trigger condition, no predefined outputs).
    // Narrowest faithful interpretation: do not invent a trigger or outputs; proceed with the algorithm.

    // Read connections: each has (u, v, cost, time).
    // The spec does not state directed/undirected; narrowest interpretation: treat as directed u -> v only.
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        let c: i64 = it.next().unwrap().parse().unwrap();
        let t: i64 = it.next().unwrap().parse().unwrap();
        if u < n && v < n {
            adj[u].push(Edge { to: v, cost: c, time: t });
        }
        // If indices are invalid, the spec says inputs are valid; we ignore invalid edges rather than panic.
    }

    // Additional per-city data: (cost, time) for each city.
    // The spec does not define how this data affects transitions; narrowest interpretation: read and ignore.
    for _ in 0..n {
        if it.next().is_none() {
            break;
        }
        let _city_cost: i64 = it.next().unwrap().parse().unwrap();
        let _city_time: i64 = it.next().unwrap().parse().unwrap();
    }

    // Modified Dijkstra with budget constraint:
    // State: (time, node, spent_cost). We minimize time subject to spent_cost <= budget.
    // Since budget can be large/unspecified, we keep a Pareto frontier per node:
    // best[node] maps spent_cost -> min_time, but stored as a small vector of non-dominated states.
    #[derive(Clone, Copy, Debug)]
    struct State {
        time: i64,
        node: usize,
        cost: i64,
    }

    let mut heap: BinaryHeap<(Reverse<i64>, usize, i64)> = BinaryHeap::new();
    let mut frontier: Vec<Vec<(i64, i64)>> = vec![Vec::new(); n]; // (cost, time), non-dominated

    // Starting city is 0 by implication; time to start is always 0.
    frontier[0].push((0, 0));
    heap.push((Reverse(0), 0, 0));

    while let Some((Reverse(cur_time), u, cur_cost)) = heap.pop() {
        // Skip if this state is no longer present in the frontier (dominated/removed).
        if !frontier[u].iter().any(|&(c, t)| c == cur_cost && t == cur_time) {
            continue;
        }

        for e in &adj[u] {
            let next_cost = cur_cost.saturating_add(e.cost);
            if next_cost < 0 || next_cost > budget {
                continue;
            }
            let next_time = cur_time.saturating_add(e.time);
            let v = e.to;

            // Check dominance:
            // A state (c1,t1) dominates (c2,t2) if c1 <= c2 and t1 <= t2.
            // We only keep non-dominated states.
            let mut dominated = false;
            for &(c, t) in &frontier[v] {
                if c <= next_cost && t <= next_time {
                    dominated = true;
                    break;
                }
            }
            if dominated {
                continue;
            }

            // Remove states dominated by the new one.
            frontier[v].retain(|&(c, t)| !(next_cost <= c && next_time <= t));
            frontier[v].push((next_cost, next_time));
            heap.push((Reverse(next_time), v, next_cost));
        }
    }

    // Output: minimum travel time to each city from starting city, line by line.
    // If unreachable, the spec says "returns an error code"; in Rust, we exit with non-zero.
    // Narrowest interpretation: if any city is unreachable, exit(1) without printing further.
    let mut out = String::new();
    for i in 0..n {
        if i == 0 {
            out.push_str("0\n");
            continue;
        }
        if frontier[i].is_empty() {
            std::process::exit(1);
        }
        let best_time = frontier[i].iter().map(|&(_c, t)| t).min().unwrap();
        // Spec says non-negative integer; we print as-is (i64) to preserve potential negative if present.
        out.push_str(&format!("{best_time}\n"));
    }

    print!("{out}");
}
