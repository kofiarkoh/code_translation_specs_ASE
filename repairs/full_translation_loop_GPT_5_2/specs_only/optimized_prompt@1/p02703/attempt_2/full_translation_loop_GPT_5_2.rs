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

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };
    let budget: i64 = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };

    if n == 0 {
        return;
    }

    // Special-case behavior is mentioned but not specified (no trigger condition, no predefined outputs).
    // Narrowest faithful interpretation: do not invent a trigger or outputs; proceed with the algorithm.

    // Read connections: each has (u, v, cost, time).
    // The spec does not state directed/undirected; narrowest interpretation: treat as directed u -> v only.
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    for _ in 0..m {
        let (Some(u_s), Some(v_s), Some(c_s), Some(t_s)) =
            (it.next(), it.next(), it.next(), it.next())
        else {
            // Incomplete input: stop reading edges.
            break;
        };
        let u: usize = u_s.parse().unwrap_or(usize::MAX);
        let v: usize = v_s.parse().unwrap_or(usize::MAX);
        let c: i64 = c_s.parse().unwrap_or(0);
        let t: i64 = t_s.parse().unwrap_or(0);

        if u < n && v < n {
            adj[u].push(Edge { to: v, cost: c, time: t });
        }
    }

    // Additional per-city data: (cost, time) for each city.
    // The spec does not define how this data affects transitions; narrowest interpretation: read and ignore.
    for _ in 0..n {
        let (Some(_c), Some(_t)) = (it.next(), it.next()) else {
            break;
        };
    }

    // Modified Dijkstra with budget constraint:
    // State: (time, node, spent_cost). We minimize time subject to spent_cost <= budget.
    // Keep a Pareto frontier per node: non-dominated (cost, time) pairs.
    let mut heap: BinaryHeap<(Reverse<i64>, usize, i64)> = BinaryHeap::new();
    let mut frontier: Vec<Vec<(i64, i64)>> = vec![Vec::new(); n]; // (cost, time)

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

            // Dominance check: existing (c,t) with c<=next_cost and t<=next_time makes new state useless.
            if frontier[v]
                .iter()
                .any(|&(c, t)| c <= next_cost && t <= next_time)
            {
                continue;
            }

            // Remove states dominated by the new one.
            frontier[v].retain(|&(c, t)| !(next_cost <= c && next_time <= t));
            frontier[v].push((next_cost, next_time));
            heap.push((Reverse(next_time), v, next_cost));
        }
    }

    // Output: minimum travel time to each city from starting city, printed line by line.
    // If unreachable, the spec says "returns an error code"; we exit with non-zero.
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
        out.push_str(&best_time.to_string());
        out.push('\n');
    }
    print!("{out}");
}
