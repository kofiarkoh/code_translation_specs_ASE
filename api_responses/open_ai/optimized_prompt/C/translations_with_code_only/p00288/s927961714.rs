// AOJ 0293 Algorithm Exam
// 2018.3.10 bal4u

use std::io::{self, BufRead};
use std::mem::MaybeUninit;

const QMAX: usize = 3005;
const INF: i32 = 0x10101010;
const MAX: usize = 110;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    s: usize,
}

struct EDGE {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct MinCostFlow {
    que: [QUE; QMAX],
    qsize: usize,
    edge: [[EDGE; MAX]; MAX],
    hi: [usize; MAX],
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [usize; MAX],
    preve: [usize; MAX],
    V: usize,
}

impl MinCostFlow {
    fn new() -> Self {
        Self {
            que: [MaybeUninit::uninit().assume_init(); QMAX],
            qsize: 0,
            edge: [[MaybeUninit::uninit().assume_init(); MAX]; MAX],
            hi: [0; MAX],
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
            V: 0,
        }
    }

    fn parent(i: usize) -> usize {
        i >> 1
    }

    fn left(i: usize) -> usize {
        i << 1
    }

    fn right(i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut min = if l < self.qsize && self.que[l].t < self.que[i].t {
            l
        } else {
            i
        };
        if r < self.qsize && self.que[r].t < self.que[min].t {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: usize, t: i32) {
        let i = self.qsize;
        self.qsize += 1;
        self.que[i].s = s;
        self.que[i].t = t;
        let mut i = i;
        while i > 0 && self.que[Self::parent(i)].t > self.que[i].t {
            self.que.swap(i, Self::parent(i));
            i = Self::parent(i);
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
            self.qsize = 0;
            self.enq(S, 0);
            self.dist[S] = 0;

            while self.qsize > 0 {
                let v = self.que[0].s;
                let t = self.que[0].t;
                self.deq();
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
                        self.enq(nv, nt);
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
                v = self.prevv[v];
                let ep = &mut self.edge[self.prevv[v]][self.preve[v]];
                if t > ep.cap {
                    t = ep.cap;
                }
            }
            *F -= t;

            ret += t * self.potential[T];
            for mut v in (1..=T).rev() {
                v = self.prevv[v];
                let ep = &mut self.edge[self.prevv[v]][self.preve[v]];
                ep.cap -= t;
                self.edge[v][ep.rev].cap += t;
            }
        }
        ret
    }
}

fn in_value() -> i32 {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap();
    let mut n = 0;
    let mut neg = false;

    if input.starts_with('-') {
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

fn make_edge(flow: &mut MinCostFlow, s: usize, mi: i32, N: usize, M: usize, source: usize, sink: usize, ax: &[i32], ay: &[i32], bx: &[i32], by: &[i32], c: &[i32]) {
    flow.hi.fill(0);
    for i in 0..N {
        flow.add_edge(source, i, 1, 0);
    }
    for i in 0..M {
        if (1 << i) & s != 0 {
            for j in 0..N {
                let x = abs(ax[j] - bx[i]) + abs(ay[j] - by[i]) - mi;
                let x = if x < 0 { 0 } else { x };
                flow.add_edge(j, N + i, 1, x);
            }
            flow.add_edge(N + i, sink, c[i], 0);
        }
    }
}

fn main() {
    let mut flow = MinCostFlow::new();
    let mut ax = [0; 101];
    let mut ay = [0; 101];
    let mut bx = [0; 6];
    let mut by = [0; 6];
    let mut c = [0; 6];
    let mut f = [0; 6];

    while let Some(n) = in_value().checked_sub(1) {
        let m = in_value();
        let b = in_value();
        for i in 0..n {
            ax[i] = in_value();
            ay[i] = in_value();
        }
        for i in 0..m {
            bx[i] = in_value();
            by[i] = in_value();
            c[i] = in_value();
            f[i] = in_value();
        }

        let source = n + m;
        let sink = source + 1;
        flow.V = sink + 1;

        let mut ans = INF;
        let lim = 1 << m;
        for s in 0..lim {
            let mut res = 0;
            let mut cnt = 0;
            for i in 0..m {
                if (s >> i) & 1 != 0 {
                    res += f[i];
                    cnt += 1;
                }
            }
            let mut lb = 0;
            let mut ub = 4002;
            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;
                make_edge(&mut flow, s, mi, n, m, source, sink, &ax, &ay, &bx, &by, &c);
                let a1 = flow.min_cost_flow(source, sink, &mut n) + mi * b * cnt;
                make_edge(&mut flow, s, mi - 1, n, m, source, sink, &ax, &ay, &bx, &by, &c);
                let a2 = flow.min_cost_flow(source, sink, &mut n) + (mi - 1) * b * cnt;
                if a1 < 0 || a2 < 0 {
                    break;
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }
            make_edge(&mut flow, s, lb, n, m, source, sink, &ax, &ay, &bx, &by, &c);
            res += flow.min_cost_flow(source, sink, &mut n) + lb * b * cnt;
            if ans > res {
                ans = res;
            }
        }
        println!("{}", ans);
    }
}
