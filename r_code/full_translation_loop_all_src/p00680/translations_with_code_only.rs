// AOJ 1095 KND Factory
// 2018.3.7 bal4u

use std::io::{self, Read};

const QMAX: usize = 1000;
const EPS: f64 = 1e-8;
const INF: f64 = 0x01010101 as f64;
const MAX: usize = 105;
const N: usize = 105;

#[derive(Clone)]
struct QUE {
    s: i32,
    t: f64,
}

#[derive(Clone)]
struct E {
    to: usize,
    rev: usize,
    cap: i32,
    cost: f64,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(QMAX),
            qsize: 0,
        }
    }

    fn min_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut min = i;

        if l < self.qsize && self.que[l].t < self.que[min].t {
            min = l;
        }
        if r < self.qsize && self.que[r].t < self.que[min].t {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        self.que[0] = self.que[self.qsize - 1].clone();
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: i32, t: f64) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { s, t });
        let mut i = i;

        while i > 0 {
            let parent = self.parent(i);
            if self.que[parent].t > self.que[i].t {
                self.que.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn parent(&self, i: usize) -> usize {
        i >> 1
    }

    fn left(&self, i: usize) -> usize {
        i << 1
    }

    fn right(&self, i: usize) -> usize {
        (i << 1) + 1
    }
}

struct Graph {
    edge: Vec<Vec<E>>,
    hi: Vec<usize>,
    lim: Vec<usize>,
    potential: Vec<f64>,
    min_cost: Vec<f64>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
    V: usize,
}

impl Graph {
    fn new(v: usize) -> Self {
        Graph {
            edge: vec![Vec::new(); v],
            hi: vec![0; v],
            lim: vec![0; v],
            potential: vec![0.0; v],
            min_cost: vec![INF; v],
            prevv: vec![0; v],
            preve: vec![0; v],
            V: v,
        }
    }

    fn check_lim(&mut self, a: usize) {
        if self.hi[a] < self.lim[a] {
            return;
        }
        if self.lim[a] == 0 {
            self.lim[a] = 32;
            self.hi[a] = 0;
            self.edge[a].resize(self.lim[a], E { to: 0, rev: 0, cap: 0, cost: 0.0 });
        } else if self.hi[a] == self.lim[a] {
            let k = self.lim[a].min(128);
            self.lim[a] += k;
            self.edge[a].resize(self.lim[a], E { to: 0, rev: 0, cap: 0, cost: 0.0 });
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: f64) {
        self.check_lim(from);
        self.check_lim(to);
        let f = self.hi[from];
        let t = self.hi[to];
        self.hi[from] += 1;
        self.hi[to] += 1;

        self.edge[from].push(E { to, rev: t, cap, cost });
        self.edge[to].push(E { to: from, rev: f, cap: 0, cost: -cost });
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, F: &mut i32) -> f64 {
        let mut ret = 0.0;
        self.potential.fill(0.0);

        while *F > 0 {
            self.min_cost.fill(INF);
            let mut heap = MinHeap::new();
            heap.enq(S as i32, 0.0);
            self.min_cost[S] = 0.0;

            while heap.qsize > 0 {
                let s = heap.que[0].s as usize;
                let t = heap.que[0].t;
                heap.deq();
                if self.min_cost[s] < t {
                    continue;
                }

                for i in 0..self.hi[s] {
                    let ep = &self.edge[s][i];
                    let ns = ep.to;
                    let nt = t + ep.cost + self.potential[s] - self.potential[ns];
                    if ep.cap > 0 && self.min_cost[ns] > nt + EPS {
                        self.min_cost[ns] = nt;
                        self.prevv[ns] = s;
                        self.preve[ns] = i;
                        heap.enq(ns as i32, nt);
                    }
                }
            }
            if self.min_cost[T] == INF {
                return -1.0;
            }
            for s in 0..self.V {
                self.potential[s] += self.min_cost[s];
            }

            let mut k = *F;
            let mut s = T;
            while s != S {
                let ep = &self.edge[self.prevv[s]][self.preve[s]];
                if k > ep.cap {
                    k = ep.cap;
                }
                s = self.prevv[s];
            }
            *F -= k;

            ret += k as f64 * self.potential[T];
            let mut s = T;
            while s != S {
                let ep = &mut self.edge[self.prevv[s]][self.preve[s]];
                ep.cap -= k;
                self.edge[s][ep.rev].cap += k;
                s = self.prevv[s];
            }
        }
        ret
    }
}

fn gauss_jordan(n: usize) -> (Vec<f64>, bool) {
    let mut B = vec![vec![0.0; N]; N];
    let mut X = vec![0.0; N];

    for i in 0..n {
        for j in 0..n {
            B[i][j] = A[i][j];
        }
        B[i][n] = C[i] as f64;
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i..n {
            if B[j][i].abs() > B[pivot][i].abs() {
                pivot = j;
            }
        }
        B.swap(i, pivot);
        if B[i][i].abs() < EPS {
            return (X, false); // error
        }

        for j in 0..=n {
            B[i][j] /= B[i][i];
        }
        for j in 0..n {
            if i != j {
                for k in i + 1..=n {
                    B[j][k] -= B[j][i] * B[i][k];
                }
            }
        }
    }
    for i in 0..n {
        X[i] = B[i][n];
    }
    (X, true)
}

fn in_int() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap() as char;

    if c == '-' {
        c = handle.bytes().next().unwrap().unwrap() as char;
        while c.is_digit(10) {
            n = 10 * n - (c as i32 - '0' as i32);
            c = handle.bytes().next().unwrap().unwrap() as char;
        }
        return n;
    }
    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = handle.bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn main() {
    let mut T = in_int();
    let mut d = vec![0; 105];
    let mut A = [[0; N]; N];
    let mut C = [0; N];

    while T > 0 {
        T -= 1;

        let mut hi = vec![0; MAX];
        let mut lim = vec![0; MAX];

        let n = in_int() as usize;
        let s = in_int() as usize;
        let t = in_int() as usize;
        let mut F = in_int();

        for i in 0..n {
            for j in 0..n {
                A[i][j] = in_int();
            }
            C[i] = in_int();
        }

        let (X, success) = gauss_jordan(n);
        if !success {
            println!("impossible");
            continue;
        }

        let mut graph = Graph::new(n);
        for i in 0..n {
            let M = in_int();
            for j in 0..M {
                d[j] = in_int();
            }
            for j in 0..M {
                graph.add_edge(i, d[j] as usize, in_int(), (X[i] - X[d[j] as usize]).abs());
            }
        }

        let ans = graph.min_cost_flow(s, t, &mut F);
        if ans < 0.0 {
            println!("impossible");
        } else {
            println!("{:.10}", ans);
        }
    }
}
