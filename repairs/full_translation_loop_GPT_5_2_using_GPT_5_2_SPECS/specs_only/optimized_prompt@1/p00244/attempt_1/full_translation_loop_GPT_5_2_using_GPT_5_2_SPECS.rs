use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const MAX_N: usize = 102;
const MAX_R: usize = 3; // remaining waivers in {0,1,2}

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    w: i32, // non-negative per spec; keep i32 to match 32-bit signed range constraint
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // no explicit malformed-input handling required
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let m: usize = it.next().unwrap().parse().unwrap();

        // Fixed upper bound implied by spec; if exceeded, behavior is not guaranteed.
        // We still allocate dynamically; this does not add extra behavior beyond spec.
        let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let w: i32 = it.next().unwrap().parse().unwrap();
            // Node indices are valid per preconditions; undirected graph.
            adj[a].push(Edge { to: b, w });
            adj[b].push(Edge { to: a, w });
        }

        // Dijkstra over expanded state space (node, remaining waivers).
        // Costs fit in i32 per spec; use i32 throughout.
        let inf: i32 = i32::MAX / 4;
        let mut dist = vec![[inf; MAX_R]; n];

        let start = 0usize;
        let goal = n - 1;

        dist[start][2] = 0;
        let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), start, 2));

        while let Some((Reverse(d), u, r)) = pq.pop() {
            if d != dist[u][r] {
                continue;
            }
            for &Edge { to: v, w } in &adj[u] {
                // Pay edge
                let nd = d.saturating_add(w);
                if nd < dist[v][r] {
                    dist[v][r] = nd;
                    pq.push((Reverse(nd), v, r));
                }
                // Waive edge if possible
                if r > 0 {
                    let nd2 = d;
                    let r2 = r - 1;
                    if nd2 < dist[v][r2] {
                        dist[v][r2] = nd2;
                        pq.push((Reverse(nd2), v, r2));
                    }
                }
            }
        }

        // Per spec: accept only goal states with remaining waivers 0 or 2.
        // Reaching goal with exactly 1 remaining waiver is not accepted.
        let ans = dist[goal][0].min(dist[goal][2]);

        // If unreachable, spec says output is not well-defined by the code.
        // We still print the computed value (likely INF) to preserve "no explicit handling".
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
