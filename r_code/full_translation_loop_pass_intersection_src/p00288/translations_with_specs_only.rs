use std::cmp::{max, min};
use std::io::{self, Read};

const MAX_N: usize = 101;
const MAX_M: usize = 5;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Facility {
    p: Point,
    cap: usize,
    open_cost: i32,
}

fn manhattan(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// Min-cost max-flow with potentials (successive shortest augmenting path).
// Costs are i32; internal distances use i64 to avoid accidental overflow in sums.
// The specification assumes 32-bit fits; we do not add extra checks.
#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct MinCostFlow {
    g: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, fr: usize, to: usize, cap: i32, cost: i32) {
        let rev_to = self.g[to].len();
        let rev_fr = self.g[fr].len();
        self.g[fr].push(Edge { to, rev: rev_to, cap, cost });
        self.g[to].push(Edge { to: fr, rev: rev_fr, cap: 0, cost: -cost });
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> Option<i64> {
        let n = self.g.len();
        let mut h = vec![0i64; n]; // potentials
        let mut prevv = vec![0usize; n];
        let mut preve = vec![0usize; n];
        let mut res: i64 = 0;

        // Dijkstra with potentials; graph is small, use O(n^2) implementation.
        while f > 0 {
            let mut dist = vec![i64::MAX / 4; n];
            let mut used = vec![false; n];
            dist[s] = 0;

            for _ in 0..n {
                let mut v = None;
                let mut best = i64::MAX / 4;
                for i in 0..n {
                    if !used[i] && dist[i] < best {
                        best = dist[i];
                        v = Some(i);
                    }
                }
                let v = match v {
                    Some(v) => v,
                    None => break,
                };
                used[v] = true;
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = dist[v] + (e.cost as i64) + h[v] - h[e.to];
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prevv[e.to] = v;
                        preve[e.to] = i;
                    }
                }
            }

            if dist[t] >= i64::MAX / 8 {
                return None; // cannot send remaining flow
            }

            for v in 0..n {
                if dist[v] < i64::MAX / 8 {
                    h[v] += dist[v];
                }
            }

            let mut d = f;
            let mut v = t;
            while v != s {
                let pv = prevv[v];
                let pe = preve[v];
                d = min(d, self.g[pv][pe].cap);
                v = pv;
            }

            f -= d;
            res += (d as i64) * h[t];

            v = t;
            while v != s {
                let pv = prevv[v];
                let pe = preve[v];
                let rev = self.g[pv][pe].rev;
                // update forward
                self.g[pv][pe].cap -= d;
                // update reverse
                self.g[v][rev].cap += d;
                v = pv;
            }
        }

        Some(res)
    }
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
        // Preconditions say n <= 101; we do not enforce beyond fixed storage assumptions.
        let m: usize = it.next().unwrap().parse().unwrap();
        let k: i32 = it.next().unwrap().parse().unwrap();

        let mut demands = Vec::with_capacity(n);
        for _ in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let y: i32 = it.next().unwrap().parse().unwrap();
            demands.push(Point { x, y });
        }

        let mut facs = Vec::with_capacity(m);
        for _ in 0..m {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let y: i32 = it.next().unwrap().parse().unwrap();
            let cap_i32: i32 = it.next().unwrap().parse().unwrap();
            let open_cost: i32 = it.next().unwrap().parse().unwrap();
            facs.push(Facility {
                p: Point { x, y },
                cap: max(0, cap_i32) as usize,
                open_cost,
            });
        }

        // Precompute all Manhattan distances.
        let mut dist = vec![vec![0i32; m]; n];
        let mut maxd: i32 = 0;
        for i in 0..n {
            for j in 0..m {
                let d = manhattan(demands[i], facs[j].p);
                dist[i][j] = d;
                if d > maxd {
                    maxd = d;
                }
            }
        }

        // The optimized integer offset parameter is not explicitly bounded in the spec.
        // Narrowest reasonable interpretation: only offsets that can change any term,
        // i.e., 0..=max Manhattan distance, since for p > maxd all assignment costs are 0
        // and the additional term grows with p (k assumed nonnegative scaling factor).
        let p_min: i32 = 0;
        let p_max: i32 = maxd;

        let mut best: i64 = i64::MAX / 4;

        // Enumerate all non-empty subsets; empty subset cannot assign any demand.
        let total_masks = 1usize << m;
        for mask in 1..total_masks {
            let mut open_sum: i64 = 0;
            let mut chosen = Vec::new();
            let mut cap_sum: usize = 0;
            for j in 0..m {
                if (mask >> j) & 1 == 1 {
                    open_sum += facs[j].open_cost as i64;
                    chosen.push(j);
                    cap_sum += facs[j].cap;
                }
            }
            if cap_sum < n {
                continue; // infeasible by capacity count
            }

            let s_count = chosen.len() as i64;

            // For each offset p, compute min assignment cost with nonnegative part,
            // then add opening costs and additional term k * (#facilities) * p.
            for p in p_min..=p_max {
                // Build flow network:
                // source -> each demand (cap 1, cost 0)
                // demand -> facility (cap 1, cost max(0, dist - p))
                // facility -> sink (cap facility.cap, cost 0)
                let nd = n;
                let nf = chosen.len();
                let src = 0usize;
                let demand_base = 1usize;
                let fac_base = demand_base + nd;
                let sink = fac_base + nf;
                let nodes = sink + 1;

                let mut mcf = MinCostFlow::new(nodes);

                for i in 0..nd {
                    mcf.add_edge(src, demand_base + i, 1, 0);
                }
                for (fi, &fj) in chosen.iter().enumerate() {
                    let cap = facs[fj].cap as i32;
                    mcf.add_edge(fac_base + fi, sink, cap, 0);
                }
                for i in 0..nd {
                    for (fi, &fj) in chosen.iter().enumerate() {
                        let c = max(0, dist[i][fj] - p);
                        mcf.add_edge(demand_base + i, fac_base + fi, 1, c);
                    }
                }

                let flow_cost = match mcf.min_cost_flow(src, sink, n as i32) {
                    Some(v) => v,
                    None => continue, // infeasible for this subset (shouldn't happen if cap_sum>=n)
                };

                let extra = (k as i64) * s_count * (p as i64);
                let total = open_sum + flow_cost + extra;
                if total < best {
                    best = total;
                }
            }
        }

        // Spec: if no subset feasible, output internal initialization-derived value.
        // We keep that behavior by printing best even if it stayed at MAX/4.
        out.push_str(&format!("{}\n", best));
    }

    print!("{out}");
}
