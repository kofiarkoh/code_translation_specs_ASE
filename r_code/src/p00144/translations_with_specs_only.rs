use std::io::{self, Read};

const MAX_N: usize = 100;
const SENTINEL: i32 = 100000;

fn compute_min_cost(adj: &Vec<Vec<i32>>, n: usize, s: usize, d: usize) -> i32 {
    // Dijkstra-like algorithm on small dense graph (O(n^2)), distances measured in edges.
    // adj[u][v] == 1 for an edge u->v, == SENTINEL for no edge.
    // We compute minimum number of edges from s to d; if reachable return edges + 1,
    // otherwise return SENTINEL.
    let mut dist = vec![SENTINEL; n];
    let mut seen = vec![false; n];

    dist[s] = 0; // zero edges to reach source itself

    for _ in 0..n {
        // select unvisited vertex with smallest dist
        let mut u: Option<usize> = None;
        let mut best = SENTINEL;
        for i in 0..n {
            if !seen[i] && dist[i] < best {
                best = dist[i];
                u = Some(i);
            }
        }
        let u = match u {
            Some(x) => x,
            None => break, // all remaining vertices unreachable
        };

        seen[u] = true;

        // relax edges u -> v
        for v in 0..n {
            if !seen[v] {
                let w = adj[u][v];
                if w < SENTINEL {
                    let tentative = dist[u].saturating_add(w);
                    if tentative < dist[v] {
                        dist[v] = tentative;
                    }
                }
            }
        }
    }

    if dist[d] >= SENTINEL {
        SENTINEL
    } else {
        // returned cost is 1 + number of edges
        dist[d].saturating_add(1)
    }
}

fn main() {
    // Read all integers from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut toks = input.split_whitespace().map(|s| s.parse::<i32>().expect("expected integer"));

    // Parse n
    let n_i32 = toks.next().expect("missing n");
    let n = n_i32 as usize;
    // Per specification, n is assumed to be 1..=100; behavior is undefined otherwise.
    // We'll trust input correctness and proceed. Cap internal structures by MAX_N to follow spec.
    let n = if n > MAX_N { n } else { n };

    // Initialize adjacency matrix with SENTINEL (no edge)
    let mut adj = vec![vec![SENTINEL; n]; n];

    // For each of the n nodes, read r1, k1, then k1 neighbors.
    for _ in 0..n {
        let r1 = toks.next().expect("missing r1") as usize;
        let k1 = toks.next().expect("missing k1") as usize;

        // Per specification, the last provided adjacency for a node defines its outgoing list.
        // Reset the row for r1.
        let row_idx = r1 - 1; // input is 1-based
        for j in 0..n {
            adj[row_idx][j] = SENTINEL;
        }

        for _ in 0..k1 {
            let neigh = toks.next().expect("missing neighbor") as usize;
            let col_idx = neigh - 1;
            // directed edge r1 -> neigh with cost 1
            adj[row_idx][col_idx] = 1;
        }
    }

    // Read p
    let p = toks.next().expect("missing p") as usize;

    let mut outputs = Vec::with_capacity(p);

    for _ in 0..p {
        let s1 = toks.next().expect("missing s1") as usize;
        let d1 = toks.next().expect("missing d1") as usize;
        let v1 = toks.next().expect("missing v1") as i32;

        let s_idx = s1 - 1;
        let d_idx = d1 - 1;

        let cost = compute_min_cost(&adj, n, s_idx, d_idx);

        if cost < SENTINEL && cost <= v1 {
            outputs.push(cost.to_string());
        } else {
            outputs.push("NA".to_string());
        }
    }

    // Print outputs, one per line
    let out = outputs.join("\n");
    println!("{}", out);
}
