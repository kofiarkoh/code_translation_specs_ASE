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
        if self.qsize > 0 {
            self.que[0] = self.que[self.qsize - 1];
            self.qsize -= 1;
            self.min_heapify(0);
        }
    }

    fn enq(&mut self, s: usize, t: i32) {
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

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct FlowNetwork {
    edges: Vec<Vec<Edge>>,
    hi: Vec<usize>,
    potential: Vec<i32>,
    dist: Vec<i32>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
    V: usize,
}

impl FlowNetwork {
    fn new(v: usize) -> Self {
        FlowNetwork {
            edges: vec![vec![]; v],
            hi: vec![0; v],
            potential: vec![0; v],
            dist: vec![0; v],
            prevv: vec![0; v],
            preve: vec![0; v],
            V: v,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let f = self.hi[from];
        let t = self.hi[to];
        self.edges[from].push(Edge { to, cap, cost, rev: t });
        self.edges[to].push(Edge { to: from, cap: 0, cost: -cost, rev: f });
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, mut F: i32) -> i32 {
        let mut ret = 0;

        while F > 0 {
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
                    let ep = &self.edges[v][i];
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

            let mut t = F;
            let mut v = T;
            while v != S {
                let ep = &self.edges[self.prevv[v]][self.preve[v]];
                if t > ep.cap {
                    t = ep.cap;
                }
                v = self.prevv[v];
            }
            F -= t;
            ret += t * self.potential[T];

            v = T;
            while v != S {
                let ep = &mut self.edges[self.prevv[v]][self.preve[v]];
                ep.cap -= t;
                self.edges[v][ep.rev].cap += t;
                v = self.prevv[v];
            }
        }
        ret
    }
}

fn read_input() -> (usize, usize, Vec<(i32, i32)>, Vec<(i32, i32, i32, i32)>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().parse().unwrap();
    
    let ax: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let coords: Vec<i32> = lines.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();
    
    let bx: Vec<(i32, i32, i32, i32)> = (0..m)
        .map(|_| {
            let params: Vec<i32> = lines.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            (params[0], params[1], params[2], params[3])
        })
        .collect();
    
    (n, m, ax, bx)
}

fn make_edge(flow_network: &mut FlowNetwork, s: usize, mi: i32, ax: &[(i32, i32)], bx: &[(i32, i32, i32, i32)], N: usize, M: usize, source: usize, sink: usize) {
    flow_network.hi.fill(0);
    for i in 0..N {
        flow_network.add_edge(source, i, 1, 0);
    }
    for i in 0..M {
        if (1 << i) & s != 0 {
            for j in 0..N {
                let x = (ax[j].0 - bx[i].0).abs() + (ax[j].1 - bx[i].1).abs() - mi;
                let cost = if x < 0 { 0 } else { x };
                flow_network.add_edge(j, N + i, 1, cost);
            }
            flow_network.add_edge(N + i, sink, bx[i].2, 0);
        }
    }
}

fn main() {
    let (N, M, ax, bx) = read_input();
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
                res += bx[i].3;
                cnt += 1;
            }
        }
        let mut lb = 0;
        let mut ub = 4002;

        while lb + 1 < ub {
            let mi = (lb + ub) >> 1;
            let mut flow_network = FlowNetwork::new(V);
            make_edge(&mut flow_network, s, mi, &ax, &bx, N, M, source, sink);
            let a1 = flow_network.min_cost_flow(source, sink, N.try_into().unwrap()) + mi * cnt as i32;
            make_edge(&mut flow_network, s, mi - 1, &ax, &bx, N, M, source, sink);
            let a2 = flow_network.min_cost_flow(source, sink, N.try_into().unwrap()) + (mi - 1) * cnt as i32;

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
        make_edge(&mut flow_network, s, lb, &ax, &bx, N, M, source, sink);
        res += flow_network.min_cost_flow(source, sink, N.try_into().unwrap()) + lb * cnt as i32;

        if ans > res {
            ans = res;
        }
    }
    println!("{}", ans);
}
