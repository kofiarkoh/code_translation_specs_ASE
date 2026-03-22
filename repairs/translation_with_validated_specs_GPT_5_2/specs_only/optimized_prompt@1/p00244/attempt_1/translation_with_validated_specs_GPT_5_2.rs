use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    w: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let m: usize = it.next().unwrap().parse().unwrap();

        let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let c: i32 = it.next().unwrap().parse().unwrap();
            // Specification assumes indices are valid; no extra checks.
            adj[a].push(Edge { to: b, w: c });
            adj[b].push(Edge { to: a, w: c });
        }

        // dist[node][rem] = best known cost to reach node with exactly rem waivers remaining.
        // rem in {0,1,2}.
        let inf: i32 = i32::MAX / 4;
        let mut dist = vec![[inf; 3]; n];

        // Start at node 0 with 2 waivers remaining, cost 0.
        dist[0][2] = 0;

        // Dijkstra over expanded state space (node, rem).
        let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), 0, 2));

        while let Some((Reverse(d), u, rem)) = pq.pop() {
            if d != dist[u][rem] {
                continue;
            }
            for &Edge { to: v, w } in &adj[u] {
                // Transition without using a waiver:
                // permitted only when remaining waivers is not exactly 1.
                if rem != 1 {
                    let nd = d.saturating_add(w);
                    if nd < dist[v][rem] {
                        dist[v][rem] = nd;
                        pq.push((Reverse(nd), v, rem));
                    }
                }

                // Transition using a waiver (if remaining waivers > 0): cost unchanged.
                if rem > 0 {
                    let nrem = rem - 1;
                    let nd = d;
                    if nd < dist[v][nrem] {
                        dist[v][nrem] = nd;
                        pq.push((Reverse(nd), v, nrem));
                    }
                }
            }
        }

        // Final state: destination with remaining waivers 0 or 2 only.
        let dest = n - 1;
        let ans = dist[dest][0].min(dist[dest][2]);

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
