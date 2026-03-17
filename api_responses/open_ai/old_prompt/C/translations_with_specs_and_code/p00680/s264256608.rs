// AOJ 1095 KND Factory
// 2018.3.7 bal4u

use std::io::{self, BufRead};
use std::f64;
use std::vec::Vec;

const QMAX: usize = 1000;
const EPS: f64 = 1e-8;
const INF: f64 = f64::MAX;
const MAX: usize = 105;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: f64,
}

struct MinHeap {
    que: Vec<(usize, f64)>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { que: Vec::new() }
    }

    fn min_heapify(&mut self, i: usize) {
        let mut min = i;
        let l = self.left(i);
        let r = self.right(i);

        if l < self.que.len() && self.que[l].1 < self.que[min].1 {
            min = l;
        }
        if r < self.que.len() && self.que[r].1 < self.que[min].1 {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) -> (usize, f64) {
        let min_elem = self.que[0];
        self.que[0] = self.que.pop().unwrap();
        self.min_heapify(0);
        min_elem
    }

    fn enq(&mut self, s: usize, t: f64) {
        self.que.push((s, t));
        let mut i = self.que.len() - 1;
        while i > 0 {
            let parent = self.parent(i);
            if self.que[parent].1 > self.que[i].1 {
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
    edges: Vec<Vec<Edge>>,
    hi: Vec<usize>,
    lim: Vec<usize>,
    V: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); n],
            hi: vec![0; n],
            lim: vec![0; n],
            V: n,
        }
    }

    fn check_lim(&mut self, a: usize) {
        if self.hi[a] < self.lim[a] {
            return;
        }
        if self.lim[a] == 0 {
            self.lim[a] = 32;
            self.hi[a] = 0;
            self.edges[a] = Vec::with_capacity(self.lim[a]);
        } else if self.hi[a] == self.lim[a] {
            let k = self.lim[a].min(128);
            self.lim[a] += k;
            self.edges[a].reserve(k);
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: f64) {
        self.check_lim(from);
        self.check_lim(to);
        let f = self.hi[from];
        let t = self.hi[to];
        self.edges[from].push(Edge { to, rev: t, cap, cost });
        self.edges[to].push(Edge { to: from, rev: f, cap: 0, cost: -cost });
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, mut F: i32) -> f64 {
        let mut ret = 0.0;
        let mut potential = vec![0.0; self.V];
        let mut min_cost = vec![INF; self.V];
        let mut prevv = vec![0; self.V];
        let mut preve = vec![0; self.V];
        let mut heap = MinHeap::new();

        while F > 0 {
            min_cost.fill(INF);
            heap = MinHeap::new();
            heap.enq(S, 0.0);
            min_cost[S] = 0.0;

            while !heap.que.is_empty() {
                let (s, t) = heap.deq();
                if min_cost[s] < t {
                    continue;
                }

                for i in 0..self.hi[s] {
                    let ep = &self.edges[s][i];
                    let ns = ep.to;
                    let nt = t + ep.cost + potential[s] - potential[ns];
                    if ep.cap > 0 && min_cost[ns] > nt + EPS {
                        min_cost[ns] = nt;
                        prevv[ns] = s;
                        preve[ns] = i;
                        heap.enq(ns, nt);
                    }
                }
            }
            if min_cost[T] == INF {
                return -1.0;
            }
            for s in 0..self.V {
                potential[s] += min_cost[s];
            }

            let mut k = F;
            let mut s = T;
            while s != S {
                let ep = &self.edges[prevv[s]][preve[s]];
                if k > ep.cap {
                    k = ep.cap;
                }
                s = prevv[s];
            }
            F -= k;
            ret += k as f64 * potential[T];
            let mut s = T;
            while s != S {
                let ep = &mut self.edges[prevv[s]][preve[s]];
                ep.cap -= k;
                self.edges[s][ep.rev].cap += k;
                s = prevv[s];
            }
        }
        ret
    }
}

fn gauss_jordan(n: usize) -> Vec<f64> {
    let mut B = vec![vec![0.0; n + 1]; n];
    let mut X = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            B[i][j] = A[i][j];
        }
        B[i][n] = C[i];
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
            return vec![]; // error
        }

        for j in i + 1..=n {
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
    X
}

fn read_int() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let T = read_int();
    for _ in 0..T {
        let n = read_int() as usize;
        let s = read_int() as usize;
        let t = read_int() as usize;
        let F = read_int();

        let mut A = vec![vec![0; MAX]; MAX];
        let mut C = vec![0; MAX];
        let mut d = vec![0; 105];

        for i in 0..n {
            for j in 0..n {
                A[i][j] = read_int();
            }
            C[i] = read_int();
        }

        let X = gauss_jordan(n);
        let mut graph = Graph::new(n);

        for i in 0..n {
            let M = read_int();
            for j in 0..M {
                d[j] = read_int();
            }
            for j in 0..M {
                graph.add_edge(i, d[j] as usize, read_int(), (X[i] - X[d[j]] as f64).abs());
            }
        }

        let ans = graph.min_cost_flow(s, t, F);
        if ans < 0.0 {
            println!("impossible");
        } else {
            println!("{:.10}", ans);
        }
    }
}
