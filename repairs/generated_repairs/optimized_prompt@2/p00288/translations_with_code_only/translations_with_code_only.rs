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
    que: [MaybeUninit<QUE>; QMAX],
    qsize: usize,
    edge: [[MaybeUninit<EDGE>; MAX]; MAX],
    hi: [usize; MAX],
    V: usize,
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [usize; MAX],
    preve: [usize; MAX],
}

impl MinCostFlow {
    fn new() -> Self {
        Self {
            que: MaybeUninit::uninit_array(),
            qsize: 0,
            edge: MaybeUninit::uninit_array(),
            hi: [0; MAX],
            V: 0,
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
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
        let mut min = if l < self.qsize && unsafe { self.que[l].assume_init_ref() }.t < unsafe { self.que[i].assume_init_ref() }.t {
            l
        } else {
            i
        };
        if r < self.qsize && unsafe { self.que[r].assume_init_ref() }.t < unsafe { self.que[min].assume_init_ref() }.t {
            min = r;
        }
        if min != i {
            unsafe {
                std::ptr::swap(&mut self.que[i], &mut self.que[min]);
            }
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        unsafe {
            self.que[0] = self.que[self.qsize - 1];
        }
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: usize, t: i32) {
        let i = self.qsize;
        self.qsize += 1;
        unsafe {
            self.que[i].as_mut_ptr().write(QUE { s, t });
        }
        let mut i = i;
        while i > 0 {
            let min = Self::parent(i);
            if unsafe { self.que[min].assume_init_ref() }.t > unsafe { self.que[i].assume_init_ref() }.t {
                unsafe {
                    std::ptr::swap(&mut self.que[i], &mut self.que[min]);
                }
                i = min;
            } else {
                break;
            }
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let f = self.hi[from];
        let t = self.hi[to];
        unsafe {
            let ep = &mut self.edge[from][f].assume_init_mut();
            ep.to = to;
            ep.cap = cap;
            ep.cost = cost;
            ep.rev = t;
            let ep = &mut self.edge[to][t].assume_init_mut();
            ep.to = from;
            ep.cap = 0;
            ep.cost = -cost;
            ep.rev = f;
        }
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
                let v = unsafe { self.que[0].assume_init_ref() }.s;
                let t = unsafe { self.que[0].assume_init_ref() }.t;
                self.deq();
                if self.dist[v] < t {
                    continue;
                }
                for i in 0..self.hi[v] {
                    let ep = unsafe { self.edge[v][i].assume_init_ref() };
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
            let mut v = T;
            while v != S {
                let ep = unsafe { self.edge[self.prevv[v]][self.preve[v]].assume_init_ref() };
                if t > ep.cap {
                    t = ep.cap;
                }
                v = self.prevv[v];
            }
            *F -= t;

            ret += t * self.potential[T];
            let mut v = T;
            while v != S {
                let ep = unsafe { self.edge[self.prevv[v]][self.preve[v]].assume_init_ref() };
                ep.cap -= t;
                let rev_ep = unsafe { self.edge[v][ep.rev].assume_init_mut() };
                rev_ep.cap += t;
                v = self.prevv[v];
            }
        }
        ret
    }
}

fn in_int() -> i32 {
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
    if neg {
        -n
    } else {
        n
    }
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
    let mut input = io::stdin().lock().lines();
    
    while let Some(Ok(line)) = input.next() {
        let mut parts = line.split_whitespace();
        let N: usize = parts.next().unwrap().parse().unwrap();
        let M: usize = parts.next().unwrap().parse().unwrap();
        let B: i32 = parts.next().unwrap().parse().unwrap();
        
        let mut ax = vec![0; N];
        let mut ay = vec![0; N];
        let mut bx = vec![0; M];
        let mut by = vec![0; M];
        let mut c = vec![0; M];
        let mut f = vec![0; M];

        for i in 0..N {
            ax[i] = in_int();
            ay[i] = in_int();
        }
        for i in 0..M {
            bx[i] = in_int();
            by[i] = in_int();
            c[i] = in_int();
            f[i] = in_int();
        }

        let source = N + M;
        let sink = source + 1;
        flow.V = sink + 1;

        let mut ans = INF;
        let lim = 1 << M;
        for s in 0..lim {
            let mut res = 0;
            let mut cnt = 0;
            for i in 0..M {
                if (s >> i) & 1 != 0 {
                    res += f[i];
                    cnt += 1;
                }
            }
            let mut lb = 0;
            let mut ub = 4002;
            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;
                make_edge(&mut flow, s, mi, N, M, source, sink, &ax, &ay, &bx, &by, &c);
                let a1 = flow.min_cost_flow(source, sink, &mut 0) + mi * B * cnt;
                make_edge(&mut flow, s, mi - 1, N, M, source, sink, &ax, &ay, &bx, &by, &c);
                let a2 = flow.min_cost_flow(source, sink, &mut 0) + (mi - 1) * B * cnt;
                if a1 < 0 || a2 < 0 {
                    break;
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }
            make_edge(&mut flow, s, lb, N, M, source, sink, &ax, &ay, &bx, &by, &c);
            res += flow.min_cost_flow(source, sink, &mut 0) + lb * B * cnt;
            if ans > res {
                ans = res;
            }
        }
        println!("{}", ans);
    }
}
