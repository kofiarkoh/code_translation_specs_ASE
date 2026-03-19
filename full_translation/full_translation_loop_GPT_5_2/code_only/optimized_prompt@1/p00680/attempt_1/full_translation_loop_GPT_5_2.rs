use std::cmp::Ordering;
use std::io::{self, Read};

const EPS: f64 = 1e-8;
const INF: f64 = 0x01010101 as f64; // matches C's integer INF promoted to double
const MAX: usize = 105;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: f64,
}

#[derive(Clone, Copy)]
struct QItem {
    s: usize,
    t: f64,
}

// Min-heap by t (like the C code's custom heap)
struct MinHeap {
    a: Vec<QItem>,
}
impl MinHeap {
    fn new() -> Self {
        Self { a: Vec::new() }
    }
    fn is_empty(&self) -> bool {
        self.a.is_empty()
    }
    fn push(&mut self, it: QItem) {
        self.a.push(it);
        let mut i = self.a.len() - 1;
        while i > 0 {
            let p = (i - 1) >> 1;
            if self.a[p].t <= self.a[i].t {
                break;
            }
            self.a.swap(p, i);
            i = p;
        }
    }
    fn pop(&mut self) -> QItem {
        let ret = self.a[0];
        let last = self.a.pop().unwrap();
        if !self.a.is_empty() {
            self.a[0] = last;
            self.min_heapify(0);
        }
        ret
    }
    fn min_heapify(&mut self, i: usize) {
        let l = (i << 1) + 1;
        let r = (i << 1) + 2;
        let mut min = i;
        if l < self.a.len() && self.a[l].t < self.a[min].t {
            min = l;
        }
        if r < self.a.len() && self.a[r].t < self.a[min].t {
            min = r;
        }
        if min != i {
            self.a.swap(i, min);
            self.min_heapify(min);
        }
    }
}

fn add_edge(g: &mut [Vec<Edge>], from: usize, to: usize, cap: i32, cost: f64) {
    let f_rev = g[to].len();
    let t_rev = g[from].len();
    g[from].push(Edge {
        to,
        rev: f_rev,
        cap,
        cost,
    });
    g[to].push(Edge {
        to: from,
        rev: t_rev,
        cap: 0,
        cost: -cost,
    });
}

fn min_cost_flow(g: &mut [Vec<Edge>], s: usize, t: usize, mut f: i32) -> f64 {
    let v = g.len();
    let mut potential = vec![0.0f64; v];
    let mut min_cost = vec![INF; v];
    let mut prevv = vec![0usize; v];
    let mut preve = vec![0usize; v];

    let mut ret = 0.0f64;

    while f > 0 {
        for x in &mut min_cost {
            *x = INF;
        }
        let mut pq = MinHeap::new();
        min_cost[s] = 0.0;
        pq.push(QItem { s, t: 0.0 });

        while !pq.is_empty() {
            let QItem { s: vtx, t: dist } = pq.pop();
            if min_cost[vtx] < dist {
                continue;
            }
            // Iterate edges
            for (i, e) in g[vtx].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                let nv = e.to;
                let nd = dist + e.cost + potential[vtx] - potential[nv];
                if min_cost[nv] > nd + EPS {
                    min_cost[nv] = nd;
                    prevv[nv] = vtx;
                    preve[nv] = i;
                    pq.push(QItem { s: nv, t: nd });
                }
            }
        }

        if (min_cost[t] - INF).abs() < f64::EPSILON {
            return -1.0;
        }

        for i in 0..v {
            potential[i] += min_cost[i];
        }

        let mut addf = f;
        let mut vtx = t;
        while vtx != s {
            let pv = prevv[vtx];
            let pe = preve[vtx];
            let cap = g[pv][pe].cap;
            if addf > cap {
                addf = cap;
            }
            vtx = pv;
        }

        f -= addf;
        ret += (addf as f64) * potential[t];

        // Apply flow along path
        let mut vtx2 = t;
        while vtx2 != s {
            let pv = prevv[vtx2];
            let pe = preve[vtx2];
            let rev = g[pv][pe].rev;
            // Borrow-splitting: do updates with indices
            let to = g[pv][pe].to;
            debug_assert_eq!(to, vtx2);

            g[pv][pe].cap -= addf;
            g[vtx2][rev].cap += addf;

            vtx2 = pv;
        }
    }

    ret
}

// Gauss-Jordan: AX = C, with A int, C int, B double augmented, X double
fn gauss_jordan(a: &[[i32; MAX]; MAX], c: &[i32; MAX], n: usize, x: &mut [f64; MAX]) -> bool {
    let mut b = [[0.0f64; MAX + 1]; MAX];

    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[i][j] as f64;
        }
        b[i][n] = c[i] as f64;
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i..n {
            if b[j][i].abs() > b[pivot][i].abs() {
                pivot = j;
            }
        }
        // swap rows i and pivot for columns 0..=n
        for j in 0..=n {
            let tmp = b[i][j];
            b[i][j] = b[pivot][j];
            b[pivot][j] = tmp;
        }
        if b[i][i].abs() < EPS {
            return false;
        }

        for j in (i + 1)..=n {
            b[i][j] /= b[i][i];
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            for k in (i + 1)..=n {
                b[j][k] -= b[j][i] * b[i][k];
            }
        }
    }

    for i in 0..n {
        x[i] = b[i][n];
    }
    true
}

// Fast integer input similar to C's getchar_unlocked-based parser.
// Assumes input is well-formed as in the original code.
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
        // Skip nothing explicitly: C code doesn't skip whitespace robustly,
        // but AOJ inputs are whitespace-separated; we emulate typical behavior.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut sign = 1i32;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + ((c & 0x0f) as i32);
            self.idx += 1;
        }
        sign * n
    }
}

fn main() {
    let mut sc = Scanner::new();

    let tcase = sc.next_i32() as usize;
    let mut out = String::new();

    for _ in 0..tcase {
        let n = sc.next_i32() as usize;
        let s = sc.next_i32() as usize;
        let t = sc.next_i32() as usize;
        let f = sc.next_i32();

        let mut a = [[0i32; MAX]; MAX];
        let mut c = [0i32; MAX];
        let mut x = [0.0f64; MAX];

        for i in 0..n {
            for j in 0..n {
                a[i][j] = sc.next_i32();
            }
            c[i] = sc.next_i32();
        }

        // C code ignores the return value; we do the same for semantics.
        let _ = gauss_jordan(&a, &c, n, &mut x);

        let mut g: Vec<Vec<Edge>> = vec![Vec::new(); n];

        let mut d = [0usize; 105];
        for i in 0..n {
            let m = sc.next_i32() as usize;
            for j in 0..m {
                d[j] = sc.next_i32() as usize;
            }
            for j in 0..m {
                let cap = sc.next_i32();
                let to = d[j];
                let cost = (x[i] - x[to]).abs();
                add_edge(&mut g, i, to, cap, cost);
            }
        }

        let ans = min_cost_flow(&mut g, s, t, f);
        if ans < 0.0 {
            out.push_str("impossible\n");
        } else {
            out.push_str(&format!("{:.10}\n", ans));
        }
    }

    print!("{out}");
}
