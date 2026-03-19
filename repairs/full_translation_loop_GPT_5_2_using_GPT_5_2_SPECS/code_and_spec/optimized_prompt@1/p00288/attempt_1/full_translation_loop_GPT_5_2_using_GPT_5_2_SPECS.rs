use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: i32 = 0x1010_1010;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct MinCostFlow {
    g: Vec<Vec<Edge>>,
    potential: Vec<i32>,
    dist: Vec<i32>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
}

impl MinCostFlow {
    fn new(v: usize) -> Self {
        Self {
            g: vec![Vec::new(); v],
            potential: vec![0; v],
            dist: vec![INF; v],
            prevv: vec![0; v],
            preve: vec![0; v],
        }
    }

    fn reset(&mut self, v: usize) {
        self.g.clear();
        self.g.resize(v, Vec::new());
        self.potential.fill(0);
        self.dist.fill(INF);
        // prevv/preve will be overwritten as needed
        if self.prevv.len() != v {
            self.prevv.resize(v, 0);
            self.preve.resize(v, 0);
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let rev_to = self.g[to].len();
        let rev_from = self.g[from].len();
        self.g[from].push(Edge {
            to,
            rev: rev_to,
            cap,
            cost,
        });
        self.g[to].push(Edge {
            to: from,
            rev: rev_from,
            cap: 0,
            cost: -cost,
        });
    }

    // Returns -1 if cannot send F flow.
    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> i32 {
        let v = self.g.len();
        self.potential.fill(0);
        let mut ret: i32 = 0;

        while f > 0 {
            self.dist.fill(INF);
            let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
            self.dist[s] = 0;
            pq.push((Reverse(0), s));

            while let Some((Reverse(d), vtx)) = pq.pop() {
                if self.dist[vtx] < d {
                    continue;
                }
                for (ei, e) in self.g[vtx].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nv = e.to;
                    // reduced cost with potentials
                    let nd = d + e.cost - self.potential[nv] + self.potential[vtx];
                    if self.dist[nv] > nd {
                        self.dist[nv] = nd;
                        self.prevv[nv] = vtx;
                        self.preve[nv] = ei;
                        pq.push((Reverse(nd), nv));
                    }
                }
            }

            if self.dist[t] == INF {
                return -1;
            }
            for i in 0..v {
                if self.dist[i] < INF {
                    self.potential[i] = self.potential[i].saturating_add(self.dist[i]);
                }
            }

            let mut addf = f;
            let mut cur = t;
            while cur != s {
                let pv = self.prevv[cur];
                let pe = self.preve[cur];
                let cap = self.g[pv][pe].cap;
                if addf > cap {
                    addf = cap;
                }
                cur = pv;
            }

            f -= addf;
            ret = ret.saturating_add(addf.saturating_mul(self.potential[t]));

            // apply
            let mut cur2 = t;
            while cur2 != s {
                let pv = self.prevv[cur2];
                let pe = self.preve[cur2];
                let rev = self.g[pv][pe].rev;

                // mutate with split borrows
                let (to, rev_idx) = {
                    let e = &self.g[pv][pe];
                    (e.to, e.rev)
                };
                debug_assert_eq!(to, cur2);
                debug_assert_eq!(rev_idx, rev);

                self.g[pv][pe].cap -= addf;
                let rev_edge_index = self.g[pv][pe].rev;
                self.g[cur2][rev_edge_index].cap += addf;

                cur2 = pv;
            }
        }

        ret
    }
}

fn abs_i32(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn main() {
    // Read all input as bytes, parse ints.
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
        let b: i32 = it.next().unwrap().parse().unwrap();

        let mut ax = vec![0i32; n];
        let mut ay = vec![0i32; n];
        for i in 0..n {
            ax[i] = it.next().unwrap().parse().unwrap();
            ay[i] = it.next().unwrap().parse().unwrap();
        }

        let mut bx = vec![0i32; m];
        let mut by = vec![0i32; m];
        let mut cap = vec![0i32; m];
        let mut fixed = vec![0i32; m];
        for i in 0..m {
            bx[i] = it.next().unwrap().parse().unwrap();
            by[i] = it.next().unwrap().parse().unwrap();
            cap[i] = it.next().unwrap().parse().unwrap();
            fixed[i] = it.next().unwrap().parse().unwrap();
        }

        let source = n + m;
        let sink = source + 1;
        let v = sink + 1;

        let mut mcf = MinCostFlow::new(v);

        let mut ans: i32 = INF;
        let lim = 1usize << m;

        for s in 0..lim {
            let mut res_fixed: i32 = 0;
            let mut cnt: i32 = 0;
            for i in 0..m {
                if ((s >> i) & 1) != 0 {
                    res_fixed = res_fixed.saturating_add(fixed[i]);
                    cnt += 1;
                }
            }

            // Binary search on offset (mi) as in C code.
            let mut lb: i32 = 0;
            let mut ub: i32 = 4002;

            // Helper to build graph for subset s and offset mi.
            let mut make_graph = |mi: i32, mcf: &mut MinCostFlow| {
                mcf.reset(v);
                for j in 0..n {
                    mcf.add_edge(source, j, 1, 0);
                }
                for i in 0..m {
                    if ((s >> i) & 1) == 0 {
                        continue;
                    }
                    for j in 0..n {
                        let mut x = abs_i32(ax[j] - bx[i]) + abs_i32(ay[j] - by[i]) - mi;
                        if x < 0 {
                            x = 0;
                        }
                        mcf.add_edge(j, n + i, 1, x);
                    }
                    mcf.add_edge(n + i, sink, cap[i], 0);
                }
            };

            let mut infeasible = false;
            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;

                make_graph(mi, &mut mcf);
                let a1 = mcf.min_cost_flow(source, sink, n as i32);
                let a1 = if a1 < 0 {
                    -1
                } else {
                    a1.saturating_add(mi.saturating_mul(b).saturating_mul(cnt))
                };

                make_graph(mi - 1, &mut mcf);
                let a2 = mcf.min_cost_flow(source, sink, n as i32);
                let a2 = if a2 < 0 {
                    -1
                } else {
                    a2.saturating_add((mi - 1).saturating_mul(b).saturating_mul(cnt))
                };

                if a1 < 0 || a2 < 0 {
                    infeasible = true;
                    break; // matches C's goto next (skip subset)
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }
            if infeasible {
                continue;
            }

            make_graph(lb, &mut mcf);
            let flow_cost = mcf.min_cost_flow(source, sink, n as i32);
            if flow_cost < 0 {
                // C code would have skipped earlier in the search, but keep consistent with spec:
                // infeasible subsets are skipped.
                continue;
            }
            let total = res_fixed
                .saturating_add(flow_cost)
                .saturating_add(lb.saturating_mul(b).saturating_mul(cnt));
            if total < ans {
                ans = total;
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
