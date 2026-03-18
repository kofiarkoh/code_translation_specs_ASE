use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cost: u64,
}

fn main() {
    // Read all input as whitespace-separated non-negative integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input: unspecified; exit quietly.
    };

    // Preconditions say n > 0; if violated, behavior is undefined. We still avoid panics here.
    if n == 0 {
        return;
    }

    // Graph: adjacency list per node.
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];

    // Input format: for each node: node_index, edge_count, then (dest, cost)*edge_count.
    // If indices are out of range, behavior is undefined by spec; we do not add extra handling.
    for _ in 0..n {
        let idx: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let mut edges = Vec::with_capacity(m);
        for _ in 0..m {
            let to: usize = it.next().unwrap().parse().unwrap();
            let cost: u64 = it.next().unwrap().parse().unwrap();
            edges.push(Edge { to, cost });
        }
        adj[idx] = edges;
    }

    // Dijkstra from source node 0 (implied by "source node is always zero after initialization").
    let source: usize = 0;

    // Use u64::MAX as the "unreachable" sentinel; costs remain initialized if unreachable.
    let mut dist: Vec<u64> = vec![u64::MAX; n];
    let mut visited: Vec<bool> = vec![false; n];
    dist[source] = 0;

    // O(n^2) Dijkstra: repeatedly pick the unvisited node with smallest dist.
    for _ in 0..n {
        let mut u: Option<usize> = None;
        let mut best: u64 = u64::MAX;

        for i in 0..n {
            if !visited[i] && dist[i] < best {
                best = dist[i];
                u = Some(i);
            }
        }

        let u = match u {
            Some(v) => v,
            None => break, // No more reachable nodes.
        };

        visited[u] = true;

        // Relax outgoing edges.
        // If overflow occurs in dist[u] + cost, wraparound is not specified; we choose saturating
        // addition to avoid panics and keep "unreachable" sentinel meaningful.
        for e in &adj[u] {
            if visited[e.to] {
                continue;
            }
            let nd = dist[u].saturating_add(e.cost);
            if nd < dist[e.to] {
                dist[e.to] = nd;
            }
        }
    }

    // Output: "index cost" per node, one per line.
    // If unreachable, print the initialized sentinel value (u64::MAX), matching "remains initialized".
    let mut out = String::new();
    for i in 0..n {
        out.push_str(&format!("{} {}\n", i, dist[i]));
    }
    print!("{out}");
}
