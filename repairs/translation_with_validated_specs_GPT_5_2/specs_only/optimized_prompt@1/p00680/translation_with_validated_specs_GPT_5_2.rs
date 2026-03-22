use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const EPS: f64 = 1e-12;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: f64,
}

struct MinCostFlow {
    g: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: f64) {
        let rev_u = self.g[v].len();
        let rev_v = self.g[u].len();
        self.g[u].push(Edge {
            to: v,
            rev: rev_u,
            cap,
            cost,
        });
        self.g[v].push(Edge {
            to: u,
            rev: rev_v,
            cap: 0,
            cost: -cost,
        });
    }

    // Successive shortest augmenting path with potentials (Johnson reweighting).
    // Costs are nonnegative in the original network, but residual edges can be negative.
    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i64) -> Option<f64> {
        let n = self.g.len();
        let mut pot = vec![0.0f64; n];
        let mut dist = vec![0.0f64; n];
        let mut prev_v = vec![0usize; n];
        let mut prev_e = vec![0usize; n];

        let mut total_cost = 0.0f64;

        while f > 0 {
            for d in dist.iter_mut() {
                *d = f64::INFINITY;
            }
            dist[s] = 0.0;

            #[derive(Copy, Clone)]
            struct State {
                d: f64,
                v: usize,
            }
            impl Eq for State {}
            impl PartialEq for State {
                fn eq(&self, other: &Self) -> bool {
                    self.d.to_bits() == other.d.to_bits() && self.v == other.v
                }
            }
            impl Ord for State {
                fn cmp(&self, other: &Self) -> Ordering {
                    // Min-heap via reversed ordering; handle NaN by treating it as equal.
                    match other.d.partial_cmp(&self.d).unwrap_or(Ordering::Equal) {
                        Ordering::Equal => self.v.cmp(&other.v),
                        ord => ord,
                    }
                }
            }
            impl PartialOrd for State {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            let mut pq = BinaryHeap::new();
            pq.push(State { d: 0.0, v: s });

            while let Some(State { d, v }) = pq.pop() {
                if d > dist[v] + EPS {
                    continue;
                }
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = dist[v] + e.cost + pot[v] - pot[e.to];
                    if nd + EPS < dist[e.to] {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = i;
                        pq.push(State { d: nd, v: e.to });
                    }
                }
            }

            if !dist[t].is_finite() {
                return None; // cannot send remaining flow
            }

            for v in 0..n {
                if dist[v].is_finite() {
                    pot[v] += dist[v];
                }
            }

            let mut add = f;
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let cap = self.g[pv][pe].cap;
                if cap < add {
                    add = cap;
                }
                v = pv;
            }

            v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let to = self.g[pv][pe].to;
                let rev = self.g[pv][pe].rev;
                let cost = self.g[pv][pe].cost;

                // Update residual capacities.
                self.g[pv][pe].cap -= add;
                self.g[to][rev].cap += add;

                total_cost += (add as f64) * cost;
                v = pv;
            }

            f -= add;
        }

        Some(total_cost)
    }
}

// Gaussian elimination with partial pivoting for a unique solution.
// Specification says uniqueness is a precondition; if violated, behavior is not defined.
// We still proceed in a best-effort manner.
fn solve_linear_system(a: Vec<Vec<f64>>, b: Vec<f64>) -> Vec<f64> {
    let n = a.len();
    let mut aug = vec![vec![0.0f64; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = b[i];
    }

    for col in 0..n {
        // Find pivot row.
        let mut pivot = col;
        let mut best = aug[col][col].abs();
        for r in (col + 1)..n {
            let v = aug[r][col].abs();
            if v > best {
                best = v;
                pivot = r;
            }
        }
        if pivot != col {
            aug.swap(pivot, col);
        }

        let piv = aug[col][col];
        // If piv is ~0, system is singular/ill-conditioned; proceed without checked contract.
        if piv.abs() > EPS {
            // Normalize pivot row.
            for j in col..=n {
                aug[col][j] /= piv;
            }
            // Eliminate other rows.
            for r in 0..n {
                if r == col {
                    continue;
                }
                let factor = aug[r][col];
                if factor.abs() <= EPS {
                    continue;
                }
                for j in col..=n {
                    aug[r][j] -= factor * aug[col][j];
                }
            }
        }
    }

    let mut x = vec![0.0f64; n];
    for i in 0..n {
        x[i] = aug[i][n];
    }
    x
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    for _case in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let s: usize = it.next().unwrap().parse().unwrap();
        let tnode: usize = it.next().unwrap().parse().unwrap();
        let req_flow: i64 = it.next().unwrap().parse().unwrap();

        // Read integer coefficient matrix and RHS vector; solve in f64.
        let mut a = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                let v: i64 = it.next().unwrap().parse().unwrap();
                a[i][j] = v as f64;
            }
        }
        let mut b = vec![0.0f64; n];
        for i in 0..n {
            let v: i64 = it.next().unwrap().parse().unwrap();
            b[i] = v as f64;
        }

        let labels = solve_linear_system(a, b);

        // Read adjacency/capacities.
        // Specification: "For each node, a list of outgoing neighbor node indices,
        // followed by a nonnegative integer capacity for each listed outgoing edge."
        // The only well-formed way without extra delimiters is:
        //   k, then k neighbor indices, then k capacities.
        // We adopt this narrow interpretation.
        let mut mcf = MinCostFlow::new(n);
        for u in 0..n {
            let k: usize = it.next().unwrap().parse().unwrap();
            let mut vs = Vec::with_capacity(k);
            for _ in 0..k {
                let v: usize = it.next().unwrap().parse().unwrap();
                vs.push(v);
            }
            for idx in 0..k {
                let cap: i64 = it.next().unwrap().parse().unwrap();
                let v = vs[idx];
                let cost = (labels[u] - labels[v]).abs();
                mcf.add_edge(u, v, cap, cost);
            }
        }

        if req_flow == 0 {
            // Minimum cost to send zero flow is zero.
            println!("0");
            continue;
        }

        match mcf.min_cost_flow(s, tnode, req_flow) {
            Some(cost) => {
                // Specification: output as a real number; no format specified.
                // Use a stable, reasonably precise representation.
                println!("{:.10}", cost);
            }
            None => {
                // Specification: "outputs a single line indicating impossibility."
                println!("impossible");
            }
        }
    }
}
