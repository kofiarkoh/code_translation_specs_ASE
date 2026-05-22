use std::cmp::Reverse;
use std::collections::VecDeque;
use std::io::{self, Read};

const INF_I64: i64 = i64::MAX / 4;

// Sentinels (spec does not define exact values; choose narrow, explicit constants).
// Main prints the negation of the solver return value on failure.
const SENTINEL_NEG_CYCLE: i64 = 1;
const SENTINEL_NO_AUGMENT: i64 = 2;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: i64,
}

struct MinCostFlow {
    g: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, fr: usize, to: usize, cap: i64, cost: i64) {
        let rev_to = self.g[to].len();
        let rev_fr = self.g[fr].len();
        self.g[fr].push(Edge { to, rev: rev_to, cap, cost });
        self.g[to].push(Edge { to: fr, rev: rev_fr, cap: 0, cost: -cost });
    }

    // Successive shortest augmenting path with potentials.
    // Returns min cost if it can send exactly `flow` units, otherwise sentinel.
    fn min_cost_flow(&mut self, s: usize, t: usize, flow: i64) -> i64 {
        let n = self.g.len();
        let mut h = vec![0i64; n]; // potentials

        // Initialization phase: detect negative-cost cycle reachable in residual graph.
        // Use Bellman-Ford from s over edges with cap>0.
        {
            let mut dist = vec![INF_I64; n];
            dist[s] = 0;
            let mut updated = false;
            for _ in 0..n {
                updated = false;
                for v in 0..n {
                    if dist[v] == INF_I64 {
                        continue;
                    }
                    for e in &self.g[v] {
                        if e.cap <= 0 {
                            continue;
                        }
                        let nd = dist[v].saturating_add(e.cost);
                        if nd < dist[e.to] {
                            dist[e.to] = nd;
                            updated = true;
                        }
                    }
                }
                if !updated {
                    break;
                }
            }
            if updated {
                return SENTINEL_NEG_CYCLE;
            }
            // Set initial potentials to 0 (narrowest choice); BF distances could be used,
            // but spec only requires sentinel on negative cycle detection.
        }

        let mut res_cost: i64 = 0;
        let mut f = flow;

        while f > 0 {
            // Dijkstra on reduced costs (cost + h[v] - h[to]) which should be non-negative.
            let mut dist = vec![INF_I64; n];
            let mut prev_v = vec![usize::MAX; n];
            let mut prev_e = vec![usize::MAX; n];

            dist[s] = 0;
            let mut pq = std::collections::BinaryHeap::<(Reverse<i64>, usize)>::new();
            pq.push((Reverse(0), s));

            while let Some((Reverse(d), v)) = pq.pop() {
                if d != dist[v] {
                    continue;
                }
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let rcost = e.cost
                        .saturating_add(h[v])
                        .saturating_sub(h[e.to]);
                    let nd = d.saturating_add(rcost);
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = i;
                        pq.push((Reverse(nd), e.to));
                    }
                }
            }

            if dist[t] == INF_I64 {
                return SENTINEL_NO_AUGMENT;
            }

            for v in 0..n {
                if dist[v] < INF_I64 {
                    h[v] = h[v].saturating_add(dist[v]);
                }
            }

            // Augment 1 unit at a time (all capacities are 1 in this model, but keep generic).
            let mut addf = f;
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                if pv == usize::MAX || pe == usize::MAX {
                    // Should not happen if dist[t] is finite, but keep narrow failure behavior.
                    return SENTINEL_NO_AUGMENT;
                }
                let cap = self.g[pv][pe].cap;
                if cap < addf {
                    addf = cap;
                }
                v = pv;
            }

            v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let rev = self.g[pv][pe].rev;
                // Update forward
                self.g[pv][pe].cap -= addf;
                // Update reverse
                self.g[v][rev].cap += addf;
                v = pv;
            }

            // True path cost is potential difference h[t] - h[s] (since dist is reduced).
            // With h[s] == 0 always (reachable), this is h[t].
            res_cost = res_cost.saturating_add(addf.saturating_mul(h[t]));
            f -= addf;
        }

        res_cost
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let s = it.next().unwrap_or("");
        let bytes = s.as_bytes();
        // Spec: each row has at least M characters; undefined otherwise.
        grid.push(bytes[..m].to_vec());
    }

    // Collect 'o' cells in fixed scan order: bottom->top, right->left.
    let mut sources: Vec<(usize, usize)> = Vec::new();
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if grid[i][j] == b'o' {
                sources.push((i, j));
            }
        }
    }

    // For each source, convert to '.' immediately, then BFS with moves:
    // right (same row, col+1) and increasing row index (row+1, same col).
    // Move allowed only if destination currently contains '.'.
    // Candidate set: up to last 100 discovered reachable cells in BFS visitation order.
    // Record candidates as (cell_index, distance).
    let mut all_candidates: Vec<Vec<(usize, i32)>> = Vec::with_capacity(sources.len());

    let mut dist = vec![vec![-1i32; m]; n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for &(si, sj) in &sources {
        // Convert 'o' to '.' before BFS.
        grid[si][sj] = b'.';

        // Reset dist
        for i in 0..n {
            for j in 0..m {
                dist[i][j] = -1;
            }
        }
        q.clear();

        dist[si][sj] = 0;
        q.push_back((si, sj));

        let mut order: Vec<(usize, usize)> = Vec::new();
        order.push((si, sj)); // discovered first

        while let Some((i, j)) = q.pop_front() {
            let d = dist[i][j];

            // Fixed neighbor expansion order is not specified beyond the two directions.
            // Use right first, then down (increasing row index), as a narrow deterministic choice.
            // Right
            if j + 1 < m && dist[i][j + 1] < 0 && grid[i][j + 1] == b'.' {
                dist[i][j + 1] = d + 1;
                q.push_back((i, j + 1));
                order.push((i, j + 1));
            }
            // Increasing row index
            if i + 1 < n && dist[i + 1][j] < 0 && grid[i + 1][j] == b'.' {
                dist[i + 1][j] = d + 1;
                q.push_back((i + 1, j));
                order.push((i + 1, j));
            }
        }

        // Take up to last 100 discovered reachable cells.
        let take = order.len().min(100);
        let start = order.len().saturating_sub(take);
        let mut cand: Vec<(usize, i32)> = Vec::with_capacity(take);
        for &(i, j) in &order[start..] {
            let idx = i * m + j;
            cand.push((idx, dist[i][j]));
        }
        all_candidates.push(cand);
    }

    // Build flow network:
    // S -> each source node (cap 1, cost 0)
    // source -> cell node for each candidate (cap 1, cost = -distance)
    // cell -> T (cap 1, cost 0)
    let num_sources = sources.len();
    let num_cells = n * m;

    let s = 0usize;
    let src_base = 1usize;
    let cell_base = src_base + num_sources;
    let t = cell_base + num_cells;
    let node_count = t + 1;

    let mut mcf = MinCostFlow::new(node_count);

    for k in 0..num_sources {
        mcf.add_edge(s, src_base + k, 1, 0);
    }
    for cell in 0..num_cells {
        mcf.add_edge(cell_base + cell, t, 1, 0);
    }
    for (k, cand) in all_candidates.iter().enumerate() {
        let u = src_base + k;
        for &(cell_idx, d) in cand {
            // Distance is non-negative; cost is negative to maximize sum of distances.
            mcf.add_edge(u, cell_base + cell_idx, 1, -(d as i64));
        }
    }

    let min_cost = mcf.min_cost_flow(s, t, num_sources as i64);
    let ans = -min_cost;
    println!("{}", ans);
}
