// AOJ 1095 KND Factory
// 2018.3.7 bal4u

use std::io::{self, Read};
use std::f64;
use std::ptr;

const QMAX: usize = 1000;
const EPS: f64 = 1e-8;
const INF: i32 = 0x01010101;
const MAX: usize = 105;

#[derive(Clone)]
struct QUE {
    s: i32,
    t: f64,
}

struct E {
    to: usize,
    rev: usize,
    cap: i32,
    cost: f64,
}

struct Graph {
    edge: Vec<Vec<E>>,
    hi: Vec<usize>,
    lim: Vec<usize>,
    V: usize,
    potential: Vec<f64>,
    min_cost: Vec<f64>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
}

impl Graph {
    fn new(v: usize) -> Self {
        Self {
            edge: vec![Vec::new(); v],
            hi: vec![0; v],
            lim: vec![0; v],
            V: v,
            potential: vec![0.0; v],
            min_cost: vec![INF as f64; v],
            prevv: vec![0; v],
            preve: vec![0; v],
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
        self.edge[from].push(E { to, cap, cost, rev: t });
        self.edge[to].push(E { to: from, cap: 0, cost: -cost, rev: f });
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> f64 {
        let mut ret = 0.0;
        self.potential.fill(0.0);

        while f > 0 {
            self.min_cost.fill(INF as f64);
            let mut que = vec![QUE { s: s as i32, t: 0.0 }];
            let mut qsize = 1;
            self.min_cost[s] = 0.0;

            while qsize > 0 {
                let s = que[0].s as usize;
                let t = que[0].t;
                que.remove(0);
                qsize -= 1;

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
                        que.push(QUE { s: ns as i32, t: nt });
                        qsize += 1;
                    }
                }
            }

            if self.min_cost[t] == INF as f64 {
                return -1.0;
            }

            for s in 0..self.V {
                self.potential[s] += self.min_cost[s];
            }

            let mut k = f;
            let mut s = t;
            while s != s {
                let ep = &self.edge[self.prevv[s]][self.preve[s]];
                if k > ep.cap {
                    k = ep.cap;
                }
                s = self.prevv[s];
            }
            f -= k;

            ret += k as f64 * self.potential[t];
            let mut s = t;
            while s != s {
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
    let mut b = vec![vec![0.0; n + 1]; n];
    let mut x = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            b[i][j] = A[i][j];
        }
        b[i][n] = C[i] as f64;
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i..n {
            if b[j][i].abs() > b[pivot][i].abs() {
                pivot = j;
            }
        }
        b.swap(i, pivot);
        if b[i][i].abs() < EPS {
            return (x, false);
        }

        for j in 0..=n {
            b[i][j] /= b[i][i];
        }
        for j in 0..n {
            if i != j {
                for k in i + 1..=n {
                    b[j][k] -= b[j][i] * b[i][k];
                }
            }
        }
    }
    for i in 0..n {
        x[i] = b[i][n];
    }
    (x, true)
}

fn in_number() -> i32 {
    let mut n = 0;
    let mut c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
    if c == '-' {
        c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
        while c.is_digit(10) {
            n = 10 * n - (c as i32 - '0' as i32);
            c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
        }
        return n;
    }
    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
    }
    n
}

const N: usize = 105;
static mut A: [[i32; N]; N] = [[0; N]; N];
static mut C: [i32; N] = [0; N];
static mut X: [f64; N] = [0.0; N];

fn main() {
    let t = in_number();
    for _ in 0..t {
        unsafe {
            let mut hi = [0; N];
            let mut lim = [0; N];
            let mut d = [0; 105];
            let mut graph = Graph::new(N);

            let n = in_number();
            let s = in_number() as usize;
            let t = in_number() as usize;
            let f = in_number();

            for i in 0..n {
                for j in 0..n {
                    A[i][j] = in_number();
                }
                C[i] = in_number();
            }

            let (x, success) = gauss_jordan(n);
            if !success {
                println!("impossible");
                continue;
            }

            for i in 0..n {
                let m = in_number();
                for j in 0..m {
                    d[j] = in_number();
                }
                for j in 0..m {
                    graph.add_edge(i, d[j] as usize, in_number(), (X[i] - X[d[j]]).abs());
                }
            }

            let ans = graph.min_cost_flow(s, t, f);
            if ans < 0.0 {
                println!("impossible");
            } else {
                println!("{:.10}", ans);
            }
        }
    }
}
