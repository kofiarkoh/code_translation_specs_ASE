use std::io::{self, Read};

const MAX_N: usize = 52;
const MAX_V: usize = 105; // as in C (enough for 2*N+2 with N<=52)
const MX_CAP: usize = 10000;

#[derive(Clone, Copy)]
struct T {
    a: [i8; 52],
}
impl Default for T {
    fn default() -> Self {
        Self { a: [0; 52] }
    }
}

struct MxPool {
    mx: Vec<T>,
    sz: usize,
    mx_zero: usize,
    mx_inf: usize,
}
impl MxPool {
    fn new() -> Self {
        // Pre-allocate to avoid reallocations; spec allows fixed bounds.
        let mut mx = vec![T::default(); MX_CAP];
        // mx_zero = 0, mx_inf = 1
        // Initialize mx_inf to "60" in all components (as in C).
        for i in 0..52 {
            mx[1].a[i] = 60;
        }
        Self {
            mx,
            sz: 2,
            mx_zero: 0,
            mx_inf: 1,
        }
    }

    fn mx_new(&mut self) -> usize {
        let id = self.sz;
        self.sz += 1;
        // Undefined behavior in C if exceeded; we keep same assumption.
        id
    }

    fn mx_clear(&mut self, idx: usize) {
        self.mx[idx] = T::default();
    }

    fn mx_add1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] = self.mx[a].a[i].wrapping_add(self.mx[b].a[i]);
        }
    }

    fn mx_add2(&mut self, s: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[s].a[i] = self.mx[a].a[i].wrapping_add(self.mx[b].a[i]);
        }
    }

    fn mx_mul2(&mut self, m: usize, a: usize, k: i32) {
        for i in 0..52 {
            self.mx[m].a[i] = (self.mx[a].a[i] as i32 * k) as i8;
        }
    }

    fn mx_cpy(&mut self, a: usize, b: usize) {
        self.mx[a] = self.mx[b];
    }

    fn mx_cmp(&self, a: usize, b: usize) -> i32 {
        for i in 0..52 {
            let k = self.mx[a].a[i] as i32 - self.mx[b].a[i] as i32;
            if k != 0 {
                return k;
            }
        }
        0
    }
}

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: usize,  // index into mx pool
    rcost: usize, // index into mx pool (reverse cost)
}

struct Graph {
    edge: Vec<Vec<Edge>>,
    v: usize,
}
impl Graph {
    fn new(v: usize) -> Self {
        Self {
            edge: vec![Vec::new(); v],
            v,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: usize, rcost: usize) {
        let f_rev = self.edge[to].len();
        let t_rev = self.edge[from].len();
        self.edge[from].push(Edge {
            to,
            rev: f_rev,
            cap,
            cost,
            rcost,
        });
        self.edge[to].push(Edge {
            to: from,
            rev: t_rev,
            cap: 0,
            cost: rcost,
            rcost: cost,
        });
    }
}

fn min_cost_flow(mx: &mut MxPool, g: &mut Graph, s: usize, t: usize, mut f: i32) -> usize {
    let v = g.v;
    let mut dist = vec![0usize; v];
    let mut prevv = vec![0usize; v];
    let mut preve = vec![0usize; v];

    // Allocate dist indices and a temp/ret as in C.
    for i in 0..v {
        dist[i] = mx.mx_new();
    }
    let temp = mx.mx_new();
    let ret = mx.mx_new();

    while f > 0 {
        for vv in 0..v {
            mx.mx_cpy(dist[vv], mx.mx_inf);
        }
        mx.mx_clear(dist[s]);

        // Bellman-Ford style relaxation until no update (matches C).
        loop {
            let mut update = false;
            for vv in 0..v {
                if mx.mx_cmp(dist[vv], mx.mx_inf) == 0 {
                    continue;
                }
                for (ei, e) in g.edge[vv].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nv = e.to;
                    mx.mx_add2(temp, dist[vv], e.cost);
                    if mx.mx_cmp(dist[nv], temp) > 0 {
                        mx.mx_cpy(dist[nv], temp);
                        prevv[nv] = vv;
                        preve[nv] = ei;
                        update = true;
                    }
                }
            }
            if !update {
                break;
            }
        }

        let mut d = f;
        let mut vv = t;
        while vv != s {
            let pv = prevv[vv];
            let ei = preve[vv];
            let cap = g.edge[pv][ei].cap;
            if d > cap {
                d = cap;
            }
            vv = pv;
        }
        f -= d;

        mx.mx_mul2(temp, dist[t], d as i32);
        mx.mx_add1(ret, temp);

        // Augment
        let mut vv2 = t;
        while vv2 != s {
            let pv = prevv[vv2];
            let ei = preve[vv2];
            let rev = g.edge[pv][ei].rev;

            // Update forward
            g.edge[pv][ei].cap -= d;
            // Update reverse
            g.edge[vv2][rev].cap += d;

            vv2 = pv;
        }
    }

    ret
}

fn main() {
    // Read all input (C uses fgets; whitespace/newlines are not significant beyond lines).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let n: usize = it
        .next()
        .unwrap_or("")
        .trim()
        .parse::<i32>()
        .unwrap_or(0)
        .max(0) as usize;

    // Spec: 0 <= N <= 52; undefined otherwise. We assume valid.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = it.next().unwrap_or("");
        // Take first N bytes; spec guarantees they are letters.
        let bytes = line.as_bytes();
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(bytes[j]);
        }
        grid.push(row);
    }

    let mut mx = MxPool::new();

    let source = n * 2;
    let sink = source + 1;
    let v = sink + 1;
    // C uses fixed MAX=105; spec requires N<=52 so v<=106, but C's MAX=105.
    // However with N<=52, v = 2*N+2 = 106, which would exceed 105 by 1.
    // This is a mismatch between C's constant and the spec's N<=52.
    // The C program would have UB at N=52; the spec says N<=52.
    // We follow the spec and allocate dynamically, avoiding the C fixed bound.
    let mut g = Graph::new(v);

    for i in 0..n {
        // source -> row i
        g.add_edge(source, i, 1, mx.mx_new(), mx.mx_new());

        for j in 0..n {
            let c = grid[i][j];
            let a = mx.mx_new();
            let b = mx.mx_new();
            if c < b'a' {
                let idx = (c - b'A') as usize;
                mx.mx[a].a[idx] = -1;
                mx.mx[b].a[idx] = 1;
            } else {
                let idx = 26 + (c - b'a') as usize;
                mx.mx[a].a[idx] = -1;
                mx.mx[b].a[idx] = 1;
            }
            // row i -> col j
            g.add_edge(i, n + j, 1, a, b);
        }

        // col i -> sink
        g.add_edge(n + i, sink, 1, mx.mx_new(), mx.mx_new());
    }

    let ans = min_cost_flow(&mut mx, &mut g, source, sink, n as i32);

    let mut out = String::new();
    for i in 0..26 {
        let mut cnt = -(mx.mx[ans].a[i] as i32);
        while cnt > 0 {
            out.push((b'A' + i as u8) as char);
            cnt -= 1;
        }
    }
    for i in 0..26 {
        let mut cnt = -(mx.mx[ans].a[i + 26] as i32);
        while cnt > 0 {
            out.push((b'a' + i as u8) as char);
            cnt -= 1;
        }
    }
    out.push('\n');
    print!("{out}");
}
