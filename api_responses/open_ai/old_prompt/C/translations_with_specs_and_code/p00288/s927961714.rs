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

    fn deq(&mut self) {
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: usize, t: i32) {
        let mut i = self.qsize;
        self.que[i] = QUE { s, t };
        self.qsize += 1;

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

#[derive(Clone, Copy)]
struct EDGE {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct FlowNetwork {
    edge: [[EDGE; MAX]; MAX],
    hi: [usize; MAX],
    V: usize,
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [usize; MAX],
    preve: [usize; MAX],
}

impl FlowNetwork {
    fn new(V: usize) -> Self {
        FlowNetwork {
            edge: [[EDGE { to: 0, rev: 0, cap: 0, cost: 0 }; MAX]; MAX],
            hi: [0; MAX],
            V,
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
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

    fn min_cost_flow(&mut self, S: usize, T: usize, F: i32) -> i32 {
        let mut ret = 0;
        self.potential.fill(0);

        let mut flow = F;
        while flow > 0 {
            self.dist.fill(INF);
            let mut heap = MinHeap::new();
            heap.enq(S, 0);
            self.dist[S] = 0;

            while heap.qsize > 0 {
                let v = heap.que[0].s;
                let t = heap.que[0].t;
                heap.deq();

                if self.dist[v] < t {
                    continue;
                }

                for i in 0..self.hi[v] {
                    let ep = self.edge[v][i];
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

            let mut t = flow;
            for mut v in T; v != S; v = self.prevv[v] {
                let ep = self.edge[self.prevv[v]][self.preve[v]];
                if t > ep.cap {
                    t = ep.cap;
                }
            }
            flow -= t;
            ret += t * self.potential[T];

            for mut v in T; v != S; v = self.prevv[v] {
                let ep = &mut self.edge[self.prevv[v]][self.preve[v]];
                ep.cap -= t;
                self.edge[v][ep.rev].cap += t;
            }
        }
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    while let Some(line) = lines.next() {
        let mut params = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let N = params.next().unwrap();
        let M = params.next().unwrap();
        let B = params.next().unwrap();

        let mut ax = vec![0; N as usize];
        let mut ay = vec![0; N as usize];
        for i in 0..N {
            let coords: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            ax[i as usize] = coords[0];
            ay[i as usize] = coords[1];
        }

        let mut bx = vec![0; M as usize];
        let mut by = vec![0; M as usize];
        let mut c = vec![0; M as usize];
        let mut f = vec![0; M as usize];
        for i in 0..M {
            let params: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            bx[i as usize] = params[0];
            by[i as usize] = params[1];
            c[i as usize] = params[2];
            f[i as usize] = params[3];
        }

        let source = N + M;
        let sink = source + 1;
        let V = sink + 1;

        let mut ans = INF;
        let lim = 1 << M;
        for s in 0..lim {
            let mut res = 0;
            let mut cnt = 0;
            for i in 0..M {
                if (s >> i) & 1 != 0 {
                    res += f[i as usize];
                    cnt += 1;
                }
            }
            let mut lb = 0;
            let mut ub = 4002;

            while lb + 1 < ub {
                let mi = (lb + ub) >> 1;
                let mut flow_network = FlowNetwork::new(V);
                make_edge(&mut flow_network, s, mi, N, source, sink, &ax, &ay, &bx, &by, &c);
                let a1 = flow_network.min_cost_flow(source, sink, N) + mi * B * cnt;

                make_edge(&mut flow_network, s, mi - 1, N, source, sink, &ax, &ay, &bx, &by, &c);
                let a2 = flow_network.min_cost_flow(source, sink, N) + (mi - 1) * B * cnt;

                if a1 < 0 || a2 < 0 {
                    break;
                }
                if a1 < a2 {
                    lb = mi;
                } else {
                    ub = mi;
                }
            }

            let mut flow_network = FlowNetwork::new(V);
            make_edge(&mut flow_network, s, lb, N, source, sink, &ax, &ay, &bx, &by, &c);
            res += flow_network.min_cost_flow(source, sink, N) + lb * B * cnt;

            if ans > res {
                ans = res;
            }
        }
        println!("{}", ans);
    }
}

fn make_edge(flow_network: &mut FlowNetwork, s: i32, mi: i32, N: i32, source: usize, sink: usize, ax: &[i32], ay: &[i32], bx: &[i32], by: &[i32], c: &[i32]) {
    flow_network.hi.fill(0);
    for i in 0..N {
        flow_network.add_edge(source, i as usize, 1, 0);
    }
    for i in 0..(1 << bx.len()) {
        if (1 << i) & s != 0 {
            for j in 0..N {
                let x = (ax[j as usize] - bx[i as usize]).abs() + (ay[j as usize] - by[i as usize]).abs() - mi;
                let cost = if x < 0 { 0 } else { x };
                flow_network.add_edge(j as usize, N as usize + i as usize, 1, cost);
            }
            flow_network.add_edge(N as usize + i as usize, sink, c[i as usize], 0);
        }
    }
}
