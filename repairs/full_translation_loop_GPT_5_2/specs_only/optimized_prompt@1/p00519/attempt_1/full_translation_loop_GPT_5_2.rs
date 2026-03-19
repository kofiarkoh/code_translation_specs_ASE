use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; if input format is incorrect, terminate with an error.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Preconditions say positive integers; we rely on that.
    // Towns are treated as 1..=n in input; internally 0..n-1.
    let mut fare: Vec<u64> = Vec::with_capacity(n);
    let mut limit: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        let f: u64 = it.next().unwrap().parse().unwrap();
        let l: usize = it.next().unwrap().parse().unwrap();
        fare.push(f);
        limit.push(l);
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let a_in: usize = it.next().unwrap().parse().unwrap();
        let b_in: usize = it.next().unwrap().parse().unwrap();
        // Connections must be valid; if not, parsing/logic will panic (error termination).
        let a = a_in - 1;
        let b = b_in - 1;
        adj[a].push(b);
        adj[b].push(a);
    }

    // Destination is the last town; start is the first town (narrowest reasonable choice).
    let start = 0usize;
    let goal = n - 1;

    // status_mincost[town][rides_used] = min cost to reach `town` having used exactly `rides_used`
    // where 0 <= rides_used <= limit[town].
    // This matches the invariant: minimum cost to reach each town with a given number of taxi rides.
    let inf: u64 = u64::MAX / 4;
    let mut status_mincost: Vec<Vec<u64>> = Vec::with_capacity(n);
    for i in 0..n {
        // If memory allocation fails, Rust will abort/panic, matching "terminates with an error".
        status_mincost.push(vec![inf; limit[i].saturating_add(1)]);
    }

    // Min-heap via Reverse: (cost, town, rides_used_at_town)
    let mut heap: BinaryHeap<Reverse<(u64, usize, usize)>> = BinaryHeap::new();

    // Start at town 0 with 0 rides used and cost 0.
    if !status_mincost[start].is_empty() {
        status_mincost[start][0] = 0;
        heap.push(Reverse((0, start, 0)));
    }

    // Dijkstra over expanded state space (town, rides_used_at_town).
    // Transition: from (u, k) to neighbor v with rides_used_at_v = k+1, cost += fare[u],
    // provided k < limit[u] and k+1 <= limit[v].
    // This models "each town has a limited number of taxi rides available" as a per-town cap
    // on how many times you can depart using that town's taxi, and also keeps the state bounded
    // by each town's own limit as required by the invariant.
    while let Some(Reverse((cost, u, used_u))) = heap.pop() {
        if used_u >= status_mincost[u].len() {
            continue;
        }
        if cost != status_mincost[u][used_u] {
            continue;
        }

        // If we reached the goal in any state, we can stop early because this is Dijkstra.
        if u == goal {
            println!("{}", cost);
            return;
        }

        // Can we take another taxi ride from u?
        if used_u >= limit[u] {
            continue;
        }

        let next_used = used_u + 1;
        let add_cost = fare[u];

        for &v in &adj[u] {
            if next_used >= status_mincost[v].len() {
                continue;
            }
            let new_cost = cost.saturating_add(add_cost);
            if new_cost < status_mincost[v][next_used] {
                status_mincost[v][next_used] = new_cost;
                heap.push(Reverse((new_cost, v, next_used)));
            }
        }
    }

    // If destination is unreachable, no output is produced.
}
