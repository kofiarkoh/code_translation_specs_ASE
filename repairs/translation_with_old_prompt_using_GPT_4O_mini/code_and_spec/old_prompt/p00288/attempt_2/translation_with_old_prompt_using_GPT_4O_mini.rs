use std::io::{self, BufRead};

const QMAX: usize = 3005;
const INF: i32 = 0x10101010;
const MAX: usize = 110;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct MinHeap {
    que: [(usize, i32); QMAX],
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: [(0, 0); QMAX],
            qsize: 0,
        }
    }

    fn min_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut min = i;

        if l < self.qsize && self.que[l].1 < self.que[min].1 {
            min = l;
        }
        if r < self.qsize && self.que[r].1 < self.que[min].1 {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) -> (usize, i32) {
        let root = self.que[0];
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
        root
    }

    fn enq(&mut self, s: usize, t: i32) {
        let mut i = self.qsize;
        self.que[i] = (s, t);
        self.qsize += 1;

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

struct FlowNetwork {
    edges: [[Edge; MAX]; MAX],
    hi: [usize; MAX],
    potential: [i32; MAX],
    dist: [i32; MAX],
    prevv: [usize; MAX],
    preve: [usize; MAX],
    v: usize,
}

impl FlowNetwork {
    fn new(v: usize) -> Self {
        FlowNetwork {
            edges: [[Edge { to: 0, rev: 0, cap: 0, cost: 0 }; MAX]; MAX],
            hi: [0; MAX],
            potential: [0; MAX],
            dist: [0; MAX],
            prevv: [0; MAX],
            preve: [0; MAX],
            v,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
        let f = self.hi[from];
        let t = self.hi[to];
        self.edges[from][f] = Edge { to, cap, cost, rev: t };
        self.edges[to][t] = Edge { to: from, cap: 0, cost: -cost, rev: f };
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> i32 {
        let mut ret = 0;
        self.potential.fill(0);

        while f > 0 {
            self.dist.fill(INF);
            let mut heap = MinHeap::new();
            heap.enq(s, 0);
            self.dist[s] = 0;

            while heap.qsize > 0 {
                let (v, t) = heap.deq();
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
            if self.dist[t] == INF {
                return -1;
            }
            for v in 0..self.v {
                self.potential[v] += self.dist[v];
            }

            let mut t = f;
            for mut v in (0..self.v).rev() {
                let ep = &self.edges[self.prevv[v]][self.preve[v]];
                if t > ep.cap {
                    t = ep.cap;
                }
            }
            f -= t;
            ret += t * self.potential[t as usize];

            for mut v in (0..self.v).rev() {
                let ep = &mut self.edges[self.prevv[v]][self.preve[v]];
                ep.cap -= t;
                self.edges[v][ep.rev].cap += t;
            }
        }
        ret
    }
}

fn read_input() -> (usize, usize, i32, Vec<(i32, i32)>, Vec<(i32, i32, i32, i32)>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let b: i32 = lines.next().unwrap().parse().unwrap();
    
    let ax: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let line = lines.next().unwrap();
            let coords: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();
    
    let bx: Vec<(i32, i32, i32, i32)> = (0..m)
        .map(|_| {
            let line = lines.next().unwrap();
            let params: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            (params[0], params[1], params[2], params[3])
        })
        .collect();
    
    (n, m, b, ax, bx)
}

fn make_edge(flow_network: &mut FlowNetwork, s: usize, mi: i32, n: usize, m: usize, ax: &[(i32, i32)], bx: &[(i32, i32, i32, i32)]) {
    flow_network.hi.fill(0);
    for i in 0..n {
        flow_network.add_edge(s, i, 1, 0);
    }
    for i in 0..m {
        if (1 << i) & s != 0 {
            for j in 0..n {
                let x = (ax[j].0 - bx[i].0).abs() + (ax[j].1 - bx[i].1).abs() - mi;
                let cost = if x < 0 { 0 } else { x };
                flow_network.add_edge(j, n + i, 1, cost);
            }
            flow_network.add_edge(n + i, n + m + 1, bx[i].2, 0);
        }
    }
}

fn main() {
    let (n, m, b, ax, bx) = read_input();
    let source = n + m;
    let sink = source + 1;
    let v = sink + 1;

    let mut ans = INF;
    let lim = 1 << m;

    for s in 0..lim {
        let mut res = 0;
        let mut cnt = 0;
        for i in 0..m {
            if (s >> i) & 1 != 0 {
                res += bx[i].3;
                cnt += 1;
            }
        }
        let mut lb = 0;
        let mut ub = 4002;

        while lb + 1 < ub {
            let mi = (lb + ub) >> 1;
            let mut flow_network = FlowNetwork::new(v);
            make_edge(&mut flow_network, s, mi, n, m, &ax, &bx);
            let a1 = flow_network.min_cost_flow(source, sink, n.try_into().unwrap()) + mi * b * cnt;
            make_edge(&mut flow_network, s, mi - 1, n, m, &ax, &bx);
            let a2 = flow_network.min_cost_flow(source, sink, n.try_into().unwrap()) + (mi - 1) * b * cnt;

            if a1 < 0 || a2 < 0 {
                break;
            }
            if a1 < a2 {
                lb = mi;
            } else {
                ub = mi;
            }
        }
        let mut flow_network = FlowNetwork::new(v);
        make_edge(&mut flow_network, s, lb, n, m, &ax, &bx);
        res += flow_network.min_cost_flow(source, sink, n.try_into().unwrap()) + lb * b * cnt;

        if ans > res {
            ans = res;
        }
    }
    println!("{}", ans);
}
