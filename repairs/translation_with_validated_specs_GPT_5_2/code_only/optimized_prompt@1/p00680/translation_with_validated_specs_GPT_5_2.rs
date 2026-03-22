use std::cmp::Ordering;
use std::io::{self, Read};

const QMAX: usize = 1000;
const EPS: f64 = 1e-8;
const INF: f64 = 0x01010101 as f64;
const MAX: usize = 105;
const N: usize = 105;

#[derive(Clone, Copy)]
struct Que {
    s: usize,
    t: f64,
}

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: f64,
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
    fn next_i32(&mut self) -> i32 {
        // Emulates the C in() behavior: assumes valid input, handles optional '-'.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut neg = false;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            neg = true;
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
        if neg { -n } else { n }
    }
}

fn min_heapify(que: &mut [Que; QMAX], qsize: usize, mut i: usize) {
    loop {
        let l = i * 2;
        let r = i * 2 + 1;
        let mut min = i;
        if l < qsize && que[l].t < que[i].t {
            min = l;
        }
        if r < qsize && que[r].t < que[min].t {
            min = r;
        }
        if min == i {
            break;
        }
        que.swap(i, min);
        i = min;
    }
}

fn deq(que: &mut [Que; QMAX], qsize: &mut usize) {
    *qsize -= 1;
    que[0] = que[*qsize];
    min_heapify(que, *qsize, 0);
}

fn enq(que: &mut [Que; QMAX], qsize: &mut usize, s: usize, t: f64) {
    let mut i = *qsize;
    *qsize += 1;
    que[i] = Que { s, t };
    while i > 0 {
        let p = i >> 1;
        if que[p].t <= que[i].t {
            break;
        }
        que.swap(i, p);
        i = p;
    }
}

fn add_edge(g: &mut Vec<Vec<Edge>>, from: usize, to: usize, cap: i32, cost: f64) {
    let f_rev = g[to].len();
    let t_rev = g[from].len();
    g[from].push(Edge { to, rev: f_rev, cap, cost });
    g[to].push(Edge { to: from, rev: t_rev, cap: 0, cost: -cost });
}

fn min_cost_flow(g: &mut Vec<Vec<Edge>>, s: usize, t: usize, mut f: i32) -> f64 {
    let v = g.len();
    let mut potential = vec![0.0f64; v];
    let mut min_cost = vec![INF; v];
    let mut prevv = vec![0usize; v];
    let mut preve = vec![0usize; v];

    let mut que = [Que { s: 0, t: 0.0 }; QMAX];
    let mut qsize: usize = 0;

    let mut ret = 0.0f64;

    while f > 0 {
        for mc in &mut min_cost {
            *mc = INF;
        }
        qsize = 0;
        enq(&mut que, &mut qsize, s, 0.0);
        min_cost[s] = 0.0;

        while qsize != 0 {
            let cur = que[0];
            deq(&mut que, &mut qsize);
            let vtx = cur.s;
            let dist = cur.t;
            if min_cost[vtx] < dist {
                continue;
            }
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
                    enq(&mut que, &mut qsize, nv, nd);
                }
            }
        }

        if (min_cost[t] - INF).abs() < f64::EPSILON {
            return -1.0;
        }

        for i in 0..v {
            potential[i] += min_cost[i];
        }

        let mut k = f;
        let mut vtx = t;
        while vtx != s {
            let pv = prevv[vtx];
            let pe = preve[vtx];
            let cap = g[pv][pe].cap;
            if k > cap {
                k = cap;
            }
            vtx = pv;
        }
        f -= k;

        ret += (k as f64) * potential[t];

        let mut vtx2 = t;
        while vtx2 != s {
            let pv = prevv[vtx2];
            let pe = preve[vtx2];
            let rev = g[pv][pe].rev;

            // Update forward edge
            g[pv][pe].cap -= k;
            // Update reverse edge
            g[vtx2][rev].cap += k;

            vtx2 = pv;
        }
    }

    ret
}

fn gauss_jordan(n: usize, a: &[[i32; N]; N], c: &[i32; N], x: &mut [f64; N]) -> bool {
    let mut b = [[0.0f64; N + 1]; N];

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
            let bij = b[j][i];
            for k in (i + 1)..=n {
                b[j][k] -= bij * b[i][k];
            }
        }
    }

    for i in 0..n {
        x[i] = b[i][n];
    }
    true
}

fn main() {
    let mut sc = Scanner::new();

    let tcase = sc.next_i32() as usize;
    let mut out = String::new();

    for _ in 0..tcase {
        let n = sc.next_i32() as usize;
        let s = sc.next_i32() as usize;
        let t = sc.next_i32() as usize;
        let f = sc.next_i32() as i32;

        let mut a = [[0i32; N]; N];
        let mut c = [0i32; N];
        let mut x = [0.0f64; N];

        for i in 0..n {
            for j in 0..n {
                a[i][j] = sc.next_i32();
            }
            c[i] = sc.next_i32();
        }

        // C code ignores the return value; we do the same (but still compute x if possible).
        let _ = gauss_jordan(n, &a, &c, &mut x);

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
