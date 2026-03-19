// AOJ 0293 Algorithm Exam
// 2018.3.10 bal4u

use std::io::{self, BufRead};

const QMAX: usize = 3005;
const INF: i32 = 0x10101010;
const MAX: usize = 110;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    s: usize,
}

#[derive(Clone, Copy)]
struct EDGE {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct MinHeap {
    que: [QUE; QMAX],
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: [QUE { t: 0, s: 0 }; QMAX],
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

    fn deq(&mut self) -> QUE {
        self.qsize -= 1;
        let last = self.que[self.qsize];
        self.que[0] = last;
        self.min_heapify(0);
        last
    }

    fn enq(&mut self, s: usize, t: i32) {
        let i = self.qsize;
        self.qsize += 1;
        self.que[i] = QUE { s, t };
        let mut i = i;
        while i > 0 {
            let min = self.parent(i);
            if self.que[min].t > self.que[i].t {
                self.que.swap(i, min);
                i = min;
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
    edge: [[EDGE; MAX]; MAX],
    hi: [usize; MAX],
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [usize; MAX],
    preve: [usize; MAX],
    V: usize,
}

impl Graph {
    fn new(v: usize) -> Self {
        Graph {
            edge: [[EDGE { to: 0, rev: 0, cap: 0, cost: 0 }; MAX]; MAX],
            hi: [0; MAX],
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
            V: v,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let f = self.hi[from];
        let t = self.hi[to];
        self.edge[from][f] = EDGE { to, cap, cost, rev: t };
        self.edge[to][t] = EDGE { to: from, cap: 0, cost: -cost, rev: f };
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, F: &mut i32) -> i32 {
        let mut ret = 0;
        self.potential.fill(0);

        while *F > 0 {
            self.dist.fill(INF);
            let mut heap = MinHeap::new();
            heap.enq(S, 0);
            self.dist[S] = 0;

            while heap.qsize > 0 {
                let v = heap.deq().s;
                let t = heap.deq().t;
                if self.dist[v] < t {
                    continue;
                }
                for i in 0..self.hi[v] {
                    let ep = &self.edge[v][i];
                    if ep.cap <= 0 {
                        continue;
                    }
                    let nv = ep.to;
                    let nt = t + ep.cost - self.potential[nv] + self.potential[v];
                    if self.dist[nv] > nt {
                        self.dist[nv] = nt;
                        self.prevv[nv] = v;
                        self.preve[nv] = i;
                        heap.enq(nv, nt);
                    }
                }
            }
            if self.dist[T] == INF {
                return -1;
            }
            for v in 0..self.V {
                self.potential[v] += self.dist[v];
            }

            let mut t = *F;
            for mut v in (1..=T).rev() {
                let ep = &self.edge[self.prevv[v]][self.preve[v]];
                if t > ep.cap {
                    t = ep.cap;
                }
            }
            *F -= t;
            ret += t * self.potential[T];
            for mut v in (1..=T).rev() {
                let ep = &mut self.edge[self.prevv[v]][self.preve[v]];
                ep.cap -= t;
                self.edge[v][ep.rev].cap += t;
            }
        }
        ret
    }
}

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap();
    let mut n = 0;
    let mut neg = false;

    if input.chars().next() == Some('-') {
        neg = true;
        input.remove(0);
    }

    for c in input.chars() {
        if c.is_digit(10) {
            n = n * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            break;
        }
    }
    if neg { -n } else { n }
}

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn make_edge(graph: &mut Graph, source: usize, s: usize, N: usize, M: usize, ax: &[i32], ay: &[i32], bx: &[i32], by: &[i32], c: &[i32], f: &[i32], mi: i32) {
    graph.hi.fill(0);
    for i in 0..N {
        graph.add_edge(source, i, 1, 0);
    }
    for i in 0..M {
        if (1 << i) & s != 0 {
            for j in 0..N {
                let x = abs(ax[j] - bx[i]) + abs(ay[j] - by[i]) - mi;
                let x = if x < 0 { 0 } else { x };
                graph.add_edge(j, N + i, 1, x);
            }
            graph.add_edge(N + i, source + 1, c[i], 0);
        }
    }
}

fn main() {
    let mut ax = [0; 101];
    let mut ay = [0; 101];
    let mut bx = [0; 6];
    let mut by = [0; 6];
    let mut c = [0; 6];
    let mut f = [0; 6];

    while let Some(n) = in_number().checked_sub(1) {
        let m = in_number();
        let b = in_number();
        for i in 0..n {
            ax[i as usize] = in_number();
            ay[i as usize] = in_number();
        }
        for i in 0..m {
            bx[i as usize] = in_number();
            by[i as usize] = in_number();
            c[i as usize] = in_number();
            f[i as usize] = in_number();
        }

        let source = n + m;
        let sink = source + 1;
        let v = sink + 1;
        let mut graph = Graph::new(v);
        let mut ans = INF;
        let lim = 1 << m;

        for s in 0..lim {
            let mut res = 0;
            let mut cnt = 0;
            for i in 0..m {
                if (s >> i) & 1 != 0 {
                    res += f[i as usize];
                    cnt += 1;
                }
            }
            let mut lb = 0;
            let mut ub = 4002;

            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;
                make_edge(&mut graph, source, s, n as usize, m as usize, &ax, &ay, &bx, &by, &c, &f, mi);
                let a1 = graph.min_cost_flow(source, sink, &mut n) + mi * b * cnt;
                make_edge(&mut graph, source, s, n as usize, m as usize, &ax, &ay, &bx, &by, &c, &f, mi - 1);
                let a2 = graph.min_cost_flow(source, sink, &mut n) + (mi - 1) * b * cnt;

                if a1 < 0 || a2 < 0 {
                    break;
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }
            make_edge(&mut graph, source, s, n as usize, m as usize, &ax, &ay, &bx, &by, &c, &f, lb);
            res += graph.min_cost_flow(source, sink, &mut n) + lb * b * cnt;
            if ans > res {
                ans = res;
            }
        }
        println!("{}", ans);
    }
}
