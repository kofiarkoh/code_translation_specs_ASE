// AOJ 1095 KND Factory
// 2018.3.7 bal4u

use std::io::{self, BufRead};
use std::f64;
use std::vec::Vec;

const QMAX: usize = 1000;
const EPS: f64 = 1e-8;
const INF: f64 = 0x01010101;
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
    potential: Vec<f64>,
    min_cost: Vec<f64>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
}

impl Graph {
    fn new(max_nodes: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); max_nodes],
            hi: vec![0; max_nodes],
            lim: vec![0; max_nodes],
            V: 0,
            potential: vec![0.0; max_nodes],
            min_cost: vec![INF; max_nodes],
            prevv: vec![0; max_nodes],
            preve: vec![0; max_nodes],
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
            self.edges[a].resize(self.lim[a], Edge { to: 0, rev: 0, cap: 0, cost: 0.0 });
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: f64) {
        self.check_lim(from);
        self.check_lim(to);
        let f = self.hi[from];
        let t = self.hi[to];
        self.edges[from].push(Edge { to, cap, cost, rev: t });
        self.edges[to].push(Edge { to: from, cap: 0, cost: -cost, rev: f });
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, F: &mut i32) -> f64 {
        let mut ret = 0.0;
        self.potential.fill(0.0);

        while *F > 0 {
            self.min_cost.fill(INF);
            let mut heap = MinHeap::new();
            heap.enq(S, 0.0);
            self.min_cost[S] = 0.0;

            while !heap.que.is_empty() {
                let (s, t) = heap.deq();
                if self.min_cost[s] < t {
                    continue;
                }

                for i in 0..self.hi[s] {
                    let ep = &self.edges[s][i];
                    let ns = ep.to;
                    let nt = t + ep.cost + self.potential[s] - self.potential[ns];
                    if ep.cap > 0 && self.min_cost[ns] > nt + EPS {
                        self.min_cost[ns] = nt;
                        self.prevv[ns] = s;
                        self.preve[ns] = i;
                        heap.enq(ns, nt);
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
                let ep = &self.edges[self.prevv[s]][self.preve[s]];
                if k > ep.cap {
                    k = ep.cap;
                }
                s = self.prevv[s];
            }
            *F -= k;
            ret += k as f64 * self.potential[T];

            let mut s = T;
            while s != S {
                let ep = &mut self.edges[self.prevv[s]][self.preve[s]];
                ep.cap -= k;
                self.edges[s][ep.rev].cap += k;
                s = self.prevv[s];
            }
        }
        ret
    }
}

fn gauss_jordan(n: usize, A: &mut [[f64; MAX]; MAX], C: &mut [f64; MAX], X: &mut [f64; MAX]) -> bool {
    let mut B = [[0.0; MAX]; MAX];
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
            return false; // error
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
    true
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
        let mut hi = vec![0; MAX];
        let mut lim = vec![0; MAX];
        let n = read_int() as usize;
        let s = read_int() as usize;
        let t = read_int() as usize;
        let mut F = read_int();
        let mut A = [[0.0; MAX]; MAX];
        let mut C = [0.0; MAX];
        let mut X = [0.0; MAX];

        for i in 0..n {
            for j in 0..n {
                A[i][j] = read_int() as f64;
            }
            C[i] = read_int() as f64;
        }

        if !gauss_jordan(n, &mut A, &mut C, &mut X) {
            println!("impossible");
            continue;
        }

        let mut graph = Graph::new(MAX);
        for i in 0..n {
            let M = read_int() as usize;
            let mut d = vec![0; M];
            for j in 0..M {
                d[j] = read_int() as usize;
            }
            for j in 0..M {
                graph.add_edge(i, d[j], read_int(), (X[i] - X[d[j]]).abs());
            }
        }

        graph.V = n;
        let ans = graph.min_cost_flow(s, t, &mut F);
        if ans < 0.0 {
            println!("impossible");
        } else {
            println!("{:.10}", ans);
        }
    }
}
