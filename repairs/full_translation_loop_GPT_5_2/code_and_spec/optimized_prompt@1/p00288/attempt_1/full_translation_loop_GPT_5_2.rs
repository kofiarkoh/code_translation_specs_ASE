use std::cmp::min;
use std::io::{self, Read};

const INF: i32 = 0x10101010;

#[derive(Clone, Copy)]
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
        // prevv/preve will be overwritten when used
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

    // Dijkstra with potentials, using a binary min-heap implemented like the C code.
    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> i32 {
        let vcnt = self.g.len();
        self.potential.fill(0);
        let mut ret: i32 = 0;

        // heap arrays
        let mut heap_s: Vec<usize> = Vec::with_capacity(4096);
        let mut heap_t: Vec<i32> = Vec::with_capacity(4096);

        while f > 0 {
            self.dist.fill(INF);

            heap_s.clear();
            heap_t.clear();

            // push (s, 0)
            self.dist[s] = 0;
            heap_s.push(s);
            heap_t.push(0);

            // heap helpers
            let mut heap_push = |hs: &mut Vec<usize>, ht: &mut Vec<i32>, node: usize, key: i32| {
                let mut i = hs.len();
                hs.push(node);
                ht.push(key);
                while i > 0 {
                    let p = (i - 1) >> 1;
                    if ht[p] <= ht[i] {
                        break;
                    }
                    hs.swap(p, i);
                    ht.swap(p, i);
                    i = p;
                }
            };

            let mut heapify = |hs: &mut Vec<usize>, ht: &mut Vec<i32>, mut i: usize| {
                let n = hs.len();
                loop {
                    let l = (i << 1) + 1;
                    let r = l + 1;
                    let mut m = i;
                    if l < n && ht[l] < ht[m] {
                        m = l;
                    }
                    if r < n && ht[r] < ht[m] {
                        m = r;
                    }
                    if m == i {
                        break;
                    }
                    hs.swap(i, m);
                    ht.swap(i, m);
                    i = m;
                }
            };

            let mut heap_pop = |hs: &mut Vec<usize>, ht: &mut Vec<i32>| -> (usize, i32) {
                let node = hs[0];
                let key = ht[0];
                let last = hs.len() - 1;
                hs[0] = hs[last];
                ht[0] = ht[last];
                hs.pop();
                ht.pop();
                if !hs.is_empty() {
                    heapify(hs, ht, 0);
                }
                (node, key)
            };

            while !heap_s.is_empty() {
                let (v, curd) = heap_pop(&mut heap_s, &mut heap_t);
                if self.dist[v] < curd {
                    continue;
                }
                // relax
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nv = e.to;
                    // nt = t + cost - potential[nv] + potential[v]
                    let nt = curd + e.cost - self.potential[nv] + self.potential[v];
                    if self.dist[nv] > nt {
                        self.dist[nv] = nt;
                        self.prevv[nv] = v;
                        self.preve[nv] = i;
                        heap_push(&mut heap_s, &mut heap_t, nv, nt);
                    }
                }
            }

            if self.dist[t] == INF {
                return -1;
            }
            for v in 0..vcnt {
                self.potential[v] = self.potential[v].saturating_add(self.dist[v]);
            }

            let mut addf = f;
            let mut v = t;
            while v != s {
                let pv = self.prevv[v];
                let pe = self.preve[v];
                let cap = self.g[pv][pe].cap;
                if addf > cap {
                    addf = cap;
                }
                v = pv;
            }
            f -= addf;

            ret = ret.saturating_add(addf.saturating_mul(self.potential[t]));

            v = t;
            while v != s {
                let pv = self.prevv[v];
                let pe = self.preve[v];
                let rev = self.g[pv][pe].rev;

                // update forward
                self.g[pv][pe].cap -= addf;
                // update reverse
                self.g[v][rev].cap += addf;

                v = pv;
            }
        }

        ret
    }
}

fn abs_i32(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_i32(&mut self) -> Option<i32> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        Some(sign * n)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let n = match sc.next_i32() {
            Some(x) => x,
            None => break,
        };
        if n == 0 {
            break; // matches C: while (N = in()) { ... }
        }
        let n = n as usize;
        let m = sc.next_i32().unwrap() as usize;
        let b = sc.next_i32().unwrap() as i32;

        let mut ax = vec![0i32; n];
        let mut ay = vec![0i32; n];
        for i in 0..n {
            ax[i] = sc.next_i32().unwrap();
            ay[i] = sc.next_i32().unwrap();
        }

        let mut bx = vec![0i32; m];
        let mut by = vec![0i32; m];
        let mut c = vec![0i32; m];
        let mut fcost = vec![0i32; m];
        for i in 0..m {
            bx[i] = sc.next_i32().unwrap();
            by[i] = sc.next_i32().unwrap();
            c[i] = sc.next_i32().unwrap();
            fcost[i] = sc.next_i32().unwrap();
        }

        let source = n + m;
        let sink = source + 1;
        let vcnt = sink + 1;

        let mut mcf = MinCostFlow::new(vcnt);

        let mut make_edge = |mcf: &mut MinCostFlow, s_mask: usize, mi: i32| {
            mcf.reset(vcnt);
            for i in 0..n {
                mcf.add_edge(source, i, 1, 0);
            }
            for i in 0..m {
                if ((1usize << i) & s_mask) != 0 {
                    for j in 0..n {
                        let mut x = abs_i32(ax[j] - bx[i]) + abs_i32(ay[j] - by[i]) - mi;
                        if x < 0 {
                            x = 0;
                        }
                        mcf.add_edge(j, n + i, 1, x);
                    }
                    mcf.add_edge(n + i, sink, c[i], 0);
                }
            }
        };

        let mut ans: i32 = INF;
        let lim = 1usize << m;

        'subset: for s_mask in 0..lim {
            let mut res: i32 = 0;
            let mut cnt: i32 = 0;
            for i in 0..m {
                if ((s_mask >> i) & 1) != 0 {
                    res = res.saturating_add(fcost[i]);
                    cnt += 1;
                }
            }

            let mut lb: i32 = 0;
            let mut ub: i32 = 4002;
            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;

                make_edge(&mut mcf, s_mask, mi);
                let a1 = mcf.min_cost_flow(source, sink, n as i32);
                let a1 = if a1 < 0 { -1 } else { a1.saturating_add(mi.saturating_mul(b).saturating_mul(cnt)) };

                make_edge(&mut mcf, s_mask, mi - 1);
                let a2 = mcf.min_cost_flow(source, sink, n as i32);
                let a2 = if a2 < 0 { -1 } else { a2.saturating_add((mi - 1).saturating_mul(b).saturating_mul(cnt)) };

                if a1 < 0 || a2 < 0 {
                    continue 'subset; // matches C's goto next
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }

            make_edge(&mut mcf, s_mask, lb);
            let flow_cost = mcf.min_cost_flow(source, sink, n as i32);
            if flow_cost < 0 {
                continue;
            }
            res = res
                .saturating_add(flow_cost)
                .saturating_add(lb.saturating_mul(b).saturating_mul(cnt));
            ans = min(ans, res);
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
