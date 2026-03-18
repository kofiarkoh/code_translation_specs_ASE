use std::cmp::Ordering;
use std::io::{self, Read};

const MAX_V: usize = 105;
const MX_ZERO: usize = 0;
const MX_INF: usize = 1;

#[derive(Clone, Copy)]
struct T {
    a: [i8; 52],
}

impl Default for T {
    fn default() -> Self {
        Self { a: [0; 52] }
    }
}

static mut MX: [T; 10000] = [T { a: [0; 52] }; 10000];
static mut SZ: usize = 0;

fn mx_new() -> usize {
    unsafe {
        let id = SZ;
        SZ += 1;
        id
    }
}
fn mx_clear(idx: usize) {
    unsafe { MX[idx] = T::default() }
}
fn mx_add1(a: usize, b: usize) {
    unsafe {
        for i in 0..52 {
            MX[a].a[i] = MX[a].a[i].wrapping_add(MX[b].a[i]);
        }
    }
}
fn mx_add2(s: usize, a: usize, b: usize) {
    unsafe {
        for i in 0..52 {
            MX[s].a[i] = MX[a].a[i].wrapping_add(MX[b].a[i]);
        }
    }
}
fn mx_mul2(m: usize, a: usize, k: i32) {
    unsafe {
        for i in 0..52 {
            MX[m].a[i] = (MX[a].a[i] as i32 * k) as i8; // matches C signed-char overflow behavior
        }
    }
}
fn mx_cpy(a: usize, b: usize) {
    unsafe { MX[a] = MX[b] }
}
fn mx_cmp(a: usize, b: usize) -> i32 {
    unsafe {
        for i in 0..52 {
            let k = MX[a].a[i] as i32 - MX[b].a[i] as i32;
            if k != 0 {
                return k;
            }
        }
    }
    0
}

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: usize,  // index into MX
    rcost: usize, // index into MX (reverse edge cost)
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            to: 0,
            rev: 0,
            cap: 0,
            cost: MX_ZERO,
            rcost: MX_ZERO,
        }
    }
}

struct Graph {
    edge: [[Edge; MAX_V]; MAX_V],
    hi: [usize; MAX_V],
    v: usize,
    dist: [usize; MAX_V],
    prevv: [usize; MAX_V],
    preve: [usize; MAX_V],
}

impl Graph {
    fn new(v: usize) -> Self {
        Self {
            edge: [[Edge::default(); MAX_V]; MAX_V],
            hi: [0; MAX_V],
            v,
            dist: [MX_ZERO; MAX_V],
            prevv: [0; MAX_V],
            preve: [0; MAX_V],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: usize, rcost: usize) {
        let f = self.hi[from];
        let t = self.hi[to];
        self.hi[from] += 1;
        self.hi[to] += 1;

        self.edge[from][f] = Edge {
            to,
            rev: t,
            cap,
            cost,
            rcost,
        };
        self.edge[to][t] = Edge {
            to: from,
            rev: f,
            cap: 0,
            cost: rcost,
            rcost: cost,
        };
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> usize {
        let tmp = mx_new();
        for i in 0..self.v {
            self.dist[i] = mx_new();
        }
        let ret = mx_new();

        while f > 0 {
            for v in 0..self.v {
                mx_cpy(self.dist[v], MX_INF);
            }
            mx_clear(self.dist[s]);

            loop {
                let mut update = false;
                for v in 0..self.v {
                    if mx_cmp(self.dist[v], MX_INF) == 0 {
                        continue;
                    }
                    for i in 0..self.hi[v] {
                        let e = self.edge[v][i];
                        if e.cap <= 0 {
                            continue;
                        }
                        let nv = e.to;
                        mx_add2(tmp, self.dist[v], e.cost);
                        if mx_cmp(self.dist[nv], tmp) > 0 {
                            mx_cpy(self.dist[nv], tmp);
                            self.prevv[nv] = v;
                            self.preve[nv] = i;
                            update = true;
                        }
                    }
                }
                if !update {
                    break;
                }
            }

            let mut d = f;
            let mut v = t;
            while v != s {
                let pv = self.prevv[v];
                let pe = self.preve[v];
                let e = self.edge[pv][pe];
                if d > e.cap {
                    d = e.cap;
                }
                v = pv;
            }
            f -= d;

            mx_mul2(tmp, self.dist[t], d as i32);
            mx_add1(ret, tmp);

            let mut v2 = t;
            while v2 != s {
                let pv = self.prevv[v2];
                let pe = self.preve[v2];
                let rev = self.edge[pv][pe].rev;

                // update forward
                self.edge[pv][pe].cap -= d;
                // update reverse
                self.edge[v2][rev].cap += d;

                v2 = pv;
            }
        }

        ret
    }
}

fn main() {
    // Input format is assumed valid per spec; undefined otherwise.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let n: usize = it.next().unwrap_or("0").trim().parse().unwrap();

    // map[i][j] is used for j in 0..N; C code assumes each line has at least N chars.
    let mut map: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = it.next().unwrap_or("");
        map.push(line.as_bytes().to_vec());
    }

    // Initialize mx_inf vector to 60 in all components, as in C.
    unsafe {
        for i in 0..52 {
            MX[MX_INF].a[i] = 60;
        }
        SZ = 2; // reserve 0 and 1
    }

    let source = n << 1;
    let sink = source + 1;
    let vcnt = sink + 1;

    let mut g = Graph::new(vcnt);

    for i in 0..n {
        g.add_edge(source, i, 1, mx_new(), mx_new());
        for j in 0..n {
            let c = map[i][j];
            let a = mx_new();
            let b = mx_new();
            unsafe {
                if c < b'a' {
                    let idx = (c - b'A') as usize;
                    MX[a].a[idx] = -1;
                    MX[b].a[idx] = 1;
                } else {
                    let idx = 26 + (c - b'a') as usize;
                    MX[a].a[idx] = -1;
                    MX[b].a[idx] = 1;
                }
            }
            g.add_edge(i, n + j, 1, a, b);
        }
        g.add_edge(n + i, sink, 1, mx_new(), mx_new());
    }

    let ans = g.min_cost_flow(source, sink, n as i32);

    let mut out = String::new();
    unsafe {
        for i in 0..26 {
            let mut b = -(MX[ans].a[i] as i32);
            while b > 0 {
                out.push((b'A' + i as u8) as char);
                b -= 1;
            }
        }
        for i in 0..26 {
            let mut b = -(MX[ans].a[i + 26] as i32);
            while b > 0 {
                out.push((b'a' + i as u8) as char);
                b -= 1;
            }
        }
    }
    out.push('\n');
    print!("{out}");
}
