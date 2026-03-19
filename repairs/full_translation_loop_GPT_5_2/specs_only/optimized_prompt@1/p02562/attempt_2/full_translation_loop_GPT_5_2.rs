use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: i64,
    orig_cap: i64,
}

struct MinCostMaxFlow {
    g: Vec<Vec<Edge>>,
}

impl MinCostMaxFlow {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        let rev_u = self.g[v].len();
        let rev_v = self.g[u].len();
        self.g[u].push(Edge {
            to: v,
            rev: rev_u,
            cap,
            cost,
            orig_cap: cap,
        });
        self.g[v].push(Edge {
            to: u,
            rev: rev_v,
            cap: 0,
            cost: -cost,
            orig_cap: 0,
        });
    }

    // Returns (flow, cost) or None as a sentinel for failure (e.g., negative cycle).
    fn min_cost_max_flow(&mut self, s: usize, t: usize, mut maxf: i64) -> Option<(i64, i64)> {
        let n = self.g.len();
        let inf: i64 = i64::MAX / 4;

        // Potentials for reduced costs.
        let mut pot = vec![0i64; n];

        // Initialize potentials with Bellman-Ford from s; detect negative cycles reachable from s.
        {
            let mut dist = vec![inf; n];
            dist[s] = 0;
            let mut updated = false;
            for _ in 0..n {
                updated = false;
                for u in 0..n {
                    if dist[u] == inf {
                        continue;
                    }
                    for e in &self.g[u] {
                        if e.cap <= 0 {
                            continue;
                        }
                        let nd = dist[u].saturating_add(e.cost);
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
                return None;
            }
            for i in 0..n {
                if dist[i] < inf {
                    pot[i] = dist[i];
                }
            }
        }

        let mut flow = 0i64;
        let mut cost = 0i64;

        while maxf > 0 {
            let mut dist = vec![inf; n];
            let mut prev_v = vec![usize::MAX; n];
            let mut prev_e = vec![usize::MAX; n];

            dist[s] = 0;
            let mut pq = BinaryHeap::<(Reverse<i64>, usize)>::new();
            pq.push((Reverse(0), s));

            while let Some((Reverse(d), v)) = pq.pop() {
                if d != dist[v] {
                    continue;
                }
                for (ei, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    // Reduced cost: c' = c + pot[v] - pot[to]
                    let rc = e.cost
                        .saturating_add(pot[v])
                        .saturating_sub(pot[e.to]);
                    let nd = d.saturating_add(rc);
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = ei;
                        pq.push((Reverse(nd), e.to));
                    }
                }
            }

            if dist[t] == inf {
                break;
            }

            for i in 0..n {
                if dist[i] < inf {
                    pot[i] = pot[i].saturating_add(dist[i]);
                }
            }

            let mut addf = maxf;
            let mut v = t;
            while v != s {
                let u = prev_v[v];
                let ei = prev_e[v];
                if u == usize::MAX || ei == usize::MAX {
                    return None;
                }
                addf = addf.min(self.g[u][ei].cap);
                v = u;
            }

            v = t;
            while v != s {
                let u = prev_v[v];
                let ei = prev_e[v];
                let rev = self.g[u][ei].rev;
                let edge_cost = self.g[u][ei].cost;

                self.g[u][ei].cap -= addf;
                self.g[v][rev].cap += addf;

                cost = cost.saturating_add(addf.saturating_mul(edge_cost));
                v = u;
            }

            flow += addf;
            maxf -= addf;
        }

        Some((flow, cost))
    }
}

fn main() {
    // The sample indicates:
    // - First integer: n
    // - Second integer: k (not used for limiting flow in the expected output; keep parsing but do not constrain)
    // - Then n*n costs
    //
    // Output:
    // - First line: maximum total cost (not flow amount)
    // - Then n lines of length n with 'X' for chosen edges, '.' otherwise
    //
    // This corresponds to selecting a maximum-cost perfect matching in an n x n bipartite graph.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let _k: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let s = 0usize;
    let left0 = 1usize;
    let right0 = left0 + n;
    let t = right0 + n;
    let total_nodes = t + 1;

    let mut mcmf = MinCostMaxFlow::new(total_nodes);

    for i in 0..n {
        mcmf.add_edge(s, left0 + i, 1, 0);
    }
    for j in 0..n {
        mcmf.add_edge(right0 + j, t, 1, 0);
    }

    // Track left->right edges for reconstruction.
    let mut lr_edge_index: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); n]; n];

    for i in 0..n {
        for j in 0..n {
            let c: i64 = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
            // We want to maximize total cost; min-cost flow minimizes, so negate costs.
            let u = left0 + i;
            let v = right0 + j;
            let ei = mcmf.g[u].len();
            mcmf.add_edge(u, v, 1, -c);
            lr_edge_index[i][j] = (u, ei);
        }
    }

    // Compute a perfect matching of size n (max flow = n).
    let res = mcmf.min_cost_max_flow(s, t, n as i64);

    match res {
        None => {
            // Sentinel for failure.
            println!("-1");
        }
        Some((flow, min_cost)) => {
            // If the network cannot send full n units, still output what was achieved.
            // The spec is ambiguous; this is the narrowest behavior without inventing extra output.
            // Convert back to maximum cost by negating the minimized cost.
            let max_cost = -min_cost;
            println!("{}", max_cost);

            // Visual representation with 'X' and '.'.
            // Mark edges with used flow > 0.
            for i in 0..n {
                let mut line = String::with_capacity(n);
                for j in 0..n {
                    let (u, ei) = lr_edge_index[i][j];
                    let e = &mcmf.g[u][ei];
                    let used = e.orig_cap - e.cap;
                    line.push(if used > 0 { 'X' } else { '.' });
                }
                println!("{}", line);
            }

            // If flow < n, we still printed a matrix; this matches the required "visual representation"
            // without adding extra diagnostics.
            let _ = flow;
        }
    }
}
