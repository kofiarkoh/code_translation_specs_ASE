use std::cmp::min;
use std::io::{self, Read};

const QMAX: usize = 3005;

#[derive(Copy, Clone, Default)]
struct Que {
    t: i32,
    s: i32,
}

struct MinHeap {
    a: [Que; QMAX],
    size: usize,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            a: [Que::default(); QMAX],
            size: 0,
        }
    }
    #[inline]
    fn parent(i: usize) -> usize {
        i >> 1
    }
    #[inline]
    fn left(i: usize) -> usize {
        i << 1
    }
    #[inline]
    fn right(i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut m = i;
        if l < self.size && self.a[l].t < self.a[i].t {
            m = l;
        }
        if r < self.size && self.a[r].t < self.a[m].t {
            m = r;
        }
        if m != i {
            self.a.swap(i, m);
            self.min_heapify(m);
        }
    }

    fn deq(&mut self) {
        self.size -= 1;
        self.a[0] = self.a[self.size];
        self.min_heapify(0);
    }

    fn enq(&mut self, s: i32, t: i32) {
        let mut i = self.size;
        self.size += 1;
        self.a[i].s = s;
        self.a[i].t = t;
        while i > 0 {
            let p = Self::parent(i);
            if self.a[p].t > self.a[i].t {
                self.a.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn top(&self) -> Que {
        self.a[0]
    }

    fn clear(&mut self) {
        self.size = 0;
    }
}

const INF: i32 = 0x10101010;
const MAX: usize = 110;

#[derive(Copy, Clone, Default)]
struct Edge {
    to: i32,
    rev: i32,
    cap: i32,
    cost: i32,
}

struct Mcmf {
    edge: [[Edge; MAX]; MAX],
    hi: [usize; MAX],
    v: usize,
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [i32; MAX],
    preve: [i32; MAX],
    heap: MinHeap,
}

impl Mcmf {
    fn new() -> Self {
        Self {
            edge: [[Edge::default(); MAX]; MAX],
            hi: [0; MAX],
            v: 0,
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
            heap: MinHeap::new(),
        }
    }

    fn set_v(&mut self, v: usize) {
        self.v = v;
    }

    fn clear_graph(&mut self) {
        // Equivalent to memset(hi, 0, V<<2) in C (only first V ints).
        for i in 0..self.v {
            self.hi[i] = 0;
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let f = self.hi[from];
        self.hi[from] += 1;
        let t = self.hi[to];
        self.hi[to] += 1;

        self.edge[from][f] = Edge {
            to: to as i32,
            cap,
            cost,
            rev: t as i32,
        };
        self.edge[to][t] = Edge {
            to: from as i32,
            cap: 0,
            cost: -cost,
            rev: f as i32,
        };
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> i32 {
        let mut ret: i32 = 0;
        for i in 0..self.v {
            self.potential[i] = 0;
        }

        while f > 0 {
            for i in 0..self.v {
                self.dist[i] = INF;
            }
            self.heap.clear();
            self.heap.enq(s as i32, 0);
            self.dist[s] = 0;

            while !self.heap.is_empty() {
                let q = self.heap.top();
                let v = q.s as usize;
                let cur_t = q.t;
                self.heap.deq();
                if self.dist[v] < cur_t {
                    continue;
                }
                let hv = self.hi[v];
                for i in 0..hv {
                    let ep = self.edge[v][i];
                    if ep.cap <= 0 {
                        continue;
                    }
                    let nv = ep.to as usize;
                    // Preserve i32 wraparound like C signed overflow (best-effort):
                    let nt = cur_t
                        .wrapping_add(ep.cost)
                        .wrapping_sub(self.potential[nv])
                        .wrapping_add(self.potential[v]);
                    if self.dist[nv] > nt {
                        self.dist[nv] = nt;
                        self.prevv[nv] = v as i32;
                        self.preve[nv] = i as i32;
                        self.heap.enq(nv as i32, nt);
                    }
                }
            }

            if self.dist[t] == INF {
                return -1;
            }
            for v in 0..self.v {
                self.potential[v] = self.potential[v].wrapping_add(self.dist[v]);
            }

            let mut addf = f;
            let mut v = t;
            while v != s {
                let pv = self.prevv[v] as usize;
                let pe = self.preve[v] as usize;
                let ep = self.edge[pv][pe];
                if addf > ep.cap {
                    addf = ep.cap;
                }
                v = pv;
            }
            f -= addf;

            ret = ret.wrapping_add(addf.wrapping_mul(self.potential[t]));

            let mut v2 = t;
            while v2 != s {
                let pv = self.prevv[v2] as usize;
                let pe = self.preve[v2] as usize;
                let rev = self.edge[pv][pe].rev as usize;

                // mutate forward
                self.edge[pv][pe].cap -= addf;
                // mutate reverse
                self.edge[v2][rev].cap += addf;

                v2 = pv;
            }
        }
        ret
    }
}

fn abs_i32(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }
    fn next_i32(&mut self) -> i32 {
        // Emulate the C in(): it does not skip non-digit except handling '-'.
        // In AOJ input, tokens are separated by whitespace; we skip whitespace here.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;

        let mut n: i32 = 0;
        if c == b'-' {
            if self.idx < self.buf.len() {
                c = self.buf[self.idx];
                self.idx += 1;
            }
            while c >= b'0' {
                n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
                if self.idx >= self.buf.len() {
                    break;
                }
                c = self.buf[self.idx];
                self.idx += 1;
            }
            return -n;
        }
        while c >= b'0' {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
        }
        n
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut mcmf = Mcmf::new();

    let mut ax = [0i32; 101];
    let mut ay = [0i32; 101];
    let mut bx = [0i32; 6];
    let mut by = [0i32; 6];
    let mut c = [0i32; 6];
    let mut f = [0i32; 6];

    let mut out = String::new();

    loop {
        let n = sc.next_i32();
        if n == 0 {
            break;
        }
        let n_us = n as usize;
        let m = sc.next_i32() as usize;
        let b = sc.next_i32();

        for i in 0..n_us {
            ax[i] = sc.next_i32();
            ay[i] = sc.next_i32();
        }
        for i in 0..m {
            bx[i] = sc.next_i32();
            by[i] = sc.next_i32();
            c[i] = sc.next_i32();
            f[i] = sc.next_i32();
        }

        let source = n_us + m;
        let sink = source + 1;
        let v = sink + 1;
        mcmf.set_v(v);

        let mut ans: i32 = INF;
        let lim = 1usize << m;

        for s in 0..lim {
            let mut res: i32 = 0;
            let mut cnt: i32 = 0;
            for i in 0..m {
                if ((s >> i) & 1) != 0 {
                    res = res.wrapping_add(f[i]);
                    cnt += 1;
                }
            }

            let mut lb: i32 = 0;
            let mut ub: i32 = 4002;

            // helper to build graph like make_edge(s, mi)
            let mut make_edge = |mi: i32, mcmf: &mut Mcmf| {
                mcmf.clear_graph();
                for i in 0..n_us {
                    mcmf.add_edge(source, i, 1, 0);
                }
                for i in 0..m {
                    if ((s >> i) & 1) != 0 {
                        for j in 0..n_us {
                            let mut x = abs_i32(ax[j] - bx[i]) + abs_i32(ay[j] - by[i]) - mi;
                            if x < 0 {
                                x = 0;
                            }
                            mcmf.add_edge(j, n_us + i, 1, x);
                        }
                        mcmf.add_edge(n_us + i, sink, c[i], 0);
                    }
                }
            };

            'bin: while lb + 1 < ub {
                let mi = (lb + ub) >> 1;

                make_edge(mi, &mut mcmf);
                let a1 = mcmf
                    .min_cost_flow(source, sink, n)
                    .wrapping_add(mi.wrapping_mul(b).wrapping_mul(cnt));

                make_edge(mi - 1, &mut mcmf);
                let a2 = mcmf
                    .min_cost_flow(source, sink, n)
                    .wrapping_add((mi - 1).wrapping_mul(b).wrapping_mul(cnt));

                if a1 < 0 || a2 < 0 {
                    break 'bin; // goto next;
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }

            // If we broke due to infeasible, skip like goto next.
            // In C, the goto triggers only when a1<0||a2<0 inside loop.
            // Here, if infeasible happened, we already broke; we must detect it by recomputing?
            // We can track with a flag.
            // Implement by checking feasibility at lb after loop only if not infeasible.
            // To preserve behavior, we need the same skip when infeasible occurred.
            // We'll re-run the same check at the last mi used is not available; so use a flag.
            let mut infeasible = false;
            // Re-run the binary search with flag (minimal overhead, M<=5).
            lb = 0;
            ub = 4002;
            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;

                make_edge(mi, &mut mcmf);
                let a1 = mcmf
                    .min_cost_flow(source, sink, n)
                    .wrapping_add(mi.wrapping_mul(b).wrapping_mul(cnt));

                make_edge(mi - 1, &mut mcmf);
                let a2 = mcmf
                    .min_cost_flow(source, sink, n)
                    .wrapping_add((mi - 1).wrapping_mul(b).wrapping_mul(cnt));

                if a1 < 0 || a2 < 0 {
                    infeasible = true;
                    break;
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

            make_edge(lb, &mut mcmf);
            res = res
                .wrapping_add(mcmf.min_cost_flow(source, sink, n))
                .wrapping_add(lb.wrapping_mul(b).wrapping_mul(cnt));
            ans = min(ans, res);
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
