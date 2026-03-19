use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: i64,
}

struct MinCostFlow {
    n: usize,
    g: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self { n, g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, fr: usize, to: usize, cap: i64, cost: i64) {
        // Costs/caps are specified non-negative for "associated with the edges",
        // but reverse edges will have negative cost as required by the algorithm.
        let fwd = Edge { to, rev: self.g[to].len(), cap, cost };
        let rev = Edge { to: fr, rev: self.g[fr].len(), cap: 0, cost: -cost };
        self.g[fr].push(fwd);
        self.g[to].push(rev);
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i64) -> Option<i64> {
        let n = self.n;
        let mut h = vec![0i64; n]; // potentials
        let mut dist = vec![0i64; n];
        let mut prevv = vec![0usize; n];
        let mut preve = vec![0usize; n];

        let mut res: i64 = 0;

        while f > 0 {
            dist.fill(i64::MAX / 4);
            dist[s] = 0;

            let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
            pq.push((Reverse(0), s));

            while let Some((Reverse(d), v)) = pq.pop() {
                if dist[v] < d {
                    continue;
                }
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    // Reduced cost with potentials.
                    let nd = d
                        .saturating_add(e.cost)
                        .saturating_add(h[v])
                        .saturating_sub(h[e.to]);
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prevv[e.to] = v;
                        preve[e.to] = i;
                        pq.push((Reverse(nd), e.to));
                    }
                }
            }

            if dist[t] >= i64::MAX / 8 {
                return None; // cannot achieve required flow
            }

            for v in 0..n {
                if dist[v] < i64::MAX / 8 {
                    h[v] = h[v].saturating_add(dist[v]);
                }
            }

            // Find bottleneck.
            let mut d = f;
            let mut v = t;
            while v != s {
                let pv = prevv[v];
                let pe = preve[v];
                let cap = self.g[pv][pe].cap;
                if cap < d {
                    d = cap;
                }
                v = pv;
            }

            // Apply flow.
            f -= d;
            res = res.saturating_add(d.saturating_mul(h[t]));

            let mut v2 = t;
            while v2 != s {
                let pv = prevv[v2];
                let pe = preve[v2];
                let rev = self.g[pv][pe].rev;

                // Update forward edge.
                self.g[pv][pe].cap -= d;
                // Update reverse edge.
                self.g[v2][rev].cap += d;

                v2 = pv;
            }
        }

        Some(res)
    }
}

fn main() {
    // Specification does not define exact input format beyond listing items.
    // Narrowest reasonable interpretation: all values are whitespace-separated integers:
    // N M
    // N lines: xi yi
    // M lines: xj yj cap cost
    // Then compute min cost to send total flow = sum(cap) from sources to sinks.
    //
    // Network construction (minimal, consistent with described data):
    // super_source -> each source node with infinite capacity, 0 cost
    // each sink node -> super_sink with capacity=cap, cost=fixed_cost
    // each source -> each sink with infinite capacity, cost = Manhattan distance between coordinates
    //
    // If total flow cannot be sent, output -1.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut src = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        src.push((x, y));
    }

    let mut sink = Vec::with_capacity(m);
    let mut total_flow: i64 = 0;
    for _ in 0..m {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let cap: i64 = it.next().unwrap().parse().unwrap();
        let fixed_cost: i64 = it.next().unwrap().parse().unwrap();
        sink.push((x, y, cap, fixed_cost));
        total_flow = total_flow.saturating_add(cap);
    }

    let s = 0usize;
    let src_base = 1usize;
    let sink_base = src_base + n;
    let t = sink_base + m;
    let nodes = t + 1;

    let mut mcf = MinCostFlow::new(nodes);

    let inf_cap: i64 = i64::MAX / 4;

    // super_source -> sources
    for i in 0..n {
        mcf.add_edge(s, src_base + i, inf_cap, 0);
    }

    // sources -> sinks with distance cost
    for i in 0..n {
        let (sx, sy) = src[i];
        for j in 0..m {
            let (tx, ty, _cap, _fc) = sink[j];
            let dist = (sx - tx).abs().saturating_add((sy - ty).abs());
            mcf.add_edge(src_base + i, sink_base + j, inf_cap, dist);
        }
    }

    // sinks -> super_sink with capacity and fixed cost per unit
    for j in 0..m {
        let (_x, _y, cap, fixed_cost) = sink[j];
        mcf.add_edge(sink_base + j, t, cap, fixed_cost);
    }

    let ans = match mcf.min_cost_flow(s, t, total_flow) {
        Some(v) => v,
        None => -1,
    };

    println!("{}", ans);
}
