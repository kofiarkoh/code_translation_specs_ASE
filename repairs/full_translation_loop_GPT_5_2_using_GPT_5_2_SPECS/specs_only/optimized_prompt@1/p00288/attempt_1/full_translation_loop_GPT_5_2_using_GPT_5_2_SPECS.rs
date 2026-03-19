use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Facility {
    p: Point,
    cap: usize,
    fixed: i64,
}

fn manhattan(a: Point, b: Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// Min-cost max-flow with successive shortest augmenting paths using potentials.
// Costs are i64; capacities are i64 (small). Assumes no negative cycles.
// This is used to compute the minimum assignment cost for a fixed facility subset
// and a fixed offset.
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

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i64) -> Option<i64> {
        let n = self.g.len();
        let mut pot = vec![0i64; n];
        let mut prev_v = vec![0usize; n];
        let mut prev_e = vec![0usize; n];
        let inf = i64::MAX / 4;
        let mut res = 0i64;

        while f > 0 {
            let mut dist = vec![inf; n];
            dist[s] = 0;
            let mut pq = BinaryHeap::<(Reverse<i64>, usize)>::new();
            pq.push((Reverse(0), s));

            while let Some((Reverse(d), v)) = pq.pop() {
                if d != dist[v] {
                    continue;
                }
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = d
                        .saturating_add(e.cost)
                        .saturating_add(pot[v])
                        .saturating_sub(pot[e.to]);
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = i;
                        pq.push((Reverse(nd), e.to));
                    }
                }
            }

            if dist[t] == inf {
                return None;
            }

            for v in 0..n {
                if dist[v] < inf {
                    pot[v] = pot[v].saturating_add(dist[v]);
                }
            }

            let mut addf = f;
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                addf = min(addf, self.g[pv][pe].cap);
                v = pv;
            }

            v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let rev = self.g[pv][pe].rev;
                let cost = self.g[pv][pe].cost;
                self.g[pv][pe].cap -= addf;
                self.g[v][rev].cap += addf;
                res = res.saturating_add(addf.saturating_mul(cost));
                v = pv;
            }

            f -= addf;
        }

        Some(res)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs = Vec::<String>::new();

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
        let mult: i64 = it.next().unwrap().parse().unwrap();

        let mut demands = Vec::with_capacity(n);
        for _ in 0..n {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            demands.push(Point { x, y });
        }

        let mut facilities = Vec::with_capacity(m);
        for _ in 0..m {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            let cap_i: i64 = it.next().unwrap().parse().unwrap();
            let fixed: i64 = it.next().unwrap().parse().unwrap();
            let cap = if cap_i <= 0 { 0 } else { cap_i as usize };
            facilities.push(Facility { p: Point { x, y }, cap, fixed });
        }

        // Precompute distances demand->facility.
        let mut dist = vec![vec![0i64; m]; n];
        let mut max_d = 0i64;
        for i in 0..n {
            for j in 0..m {
                let d = manhattan(demands[i], facilities[j].p);
                dist[i][j] = d;
                if d > max_d {
                    max_d = d;
                }
            }
        }

        // Sentinel behavior: if all subsets infeasible, we print this sentinel.
        let mut best: i64 = i64::MAX / 4;

        // Enumerate all non-empty subsets; empty subset cannot serve any demand.
        // (If n==0 we'd terminate earlier.)
        let total_masks = 1u64 << m;
        for mask in 1u64..total_masks {
            // Collect selected facilities and check total capacity.
            let mut sel = Vec::<usize>::new();
            let mut cap_sum: usize = 0;
            let mut fixed_sum: i64 = 0;
            for j in 0..m {
                if (mask >> j) & 1 == 1 {
                    sel.push(j);
                    cap_sum = cap_sum.saturating_add(facilities[j].cap);
                    fixed_sum = fixed_sum.saturating_add(facilities[j].fixed);
                }
            }
            if cap_sum < n {
                continue;
            }

            // Offset is an integer chosen to minimize:
            // fixed_sum + min_assignment_cost(offset) + offset * mult * |sel|
            //
            // The assignment cost uses max(0, dist - offset). For offset > max_d,
            // all assignment costs become 0, so objective becomes fixed_sum + offset*mult*|sel|.
            // If mult >= 0, searching offsets in [0..=max_d] is sufficient.
            // If mult < 0, the objective is unbounded below as offset increases; the spec
            // does not define behavior. We choose the narrowest reasonable interpretation:
            // still search [0..=max_d] (the only range implied by distances).
            let k = sel.len() as i64;
            let mut best_for_subset: Option<i64> = None;

            for offset in 0..=max_d {
                // Build flow network:
                // source -> each demand (cap 1, cost 0)
                // demand -> each selected facility (cap 1, cost max(0, dist - offset))
                // facility -> sink (cap facility.cap, cost 0)
                let s = 0usize;
                let demand_base = 1usize;
                let fac_base = demand_base + n;
                let t = fac_base + sel.len();
                let nodes = t + 1;

                let mut mcf = MinCostFlow::new(nodes);

                for i in 0..n {
                    mcf.add_edge(s, demand_base + i, 1, 0);
                }
                for (idx, &fj) in sel.iter().enumerate() {
                    let cap = facilities[fj].cap as i64;
                    mcf.add_edge(fac_base + idx, t, cap, 0);
                }
                for i in 0..n {
                    let di = demand_base + i;
                    for (idx, &fj) in sel.iter().enumerate() {
                        let raw = dist[i][fj].saturating_sub(offset);
                        let c = if raw < 0 { 0 } else { raw };
                        mcf.add_edge(di, fac_base + idx, 1, c);
                    }
                }

                let flow_cost = match mcf.min_cost_flow(s, t, n as i64) {
                    Some(c) => c,
                    None => continue, // infeasible for this offset (shouldn't happen if cap_sum>=n)
                };

                let linear = offset
                    .saturating_mul(mult)
                    .saturating_mul(k);
                let total = fixed_sum.saturating_add(flow_cost).saturating_add(linear);

                best_for_subset = Some(match best_for_subset {
                    None => total,
                    Some(prev) => prev.min(total),
                });
            }

            if let Some(v) = best_for_subset {
                if v < best {
                    best = v;
                }
            }
        }

        outputs.push(best.to_string());
    }

    print!("{}", outputs.join("\n"));
}
