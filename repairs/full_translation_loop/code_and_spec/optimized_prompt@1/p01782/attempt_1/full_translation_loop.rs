use std::io::{self, BufRead};

#[derive(Clone, Default)]
struct T {
    a: [i8; 52],
}

struct FlowNetwork {
    mx: Vec<T>,
    sz: usize,
    edges: Vec<Vec<Edge>>,
    hi: Vec<usize>,
    dist: Vec<usize>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
    V: usize,
}

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

impl FlowNetwork {
    fn new(max_vertices: usize) -> Self {
        Self {
            mx: vec![T::default(); 10000],
            sz: 0,
            edges: vec![vec![]; max_vertices],
            hi: vec![0; max_vertices],
            dist: vec![0; max_vertices],
            prevv: vec![0; max_vertices],
            preve: vec![0; max_vertices],
            V: max_vertices,
        }
    }

    fn new_index(&mut self) -> usize {
        let index = self.sz;
        self.sz += 1;
        index
    }

    fn clear(&mut self, a: usize) {
        self.mx[a] = T::default();
    }

    fn add1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] += self.mx[b].a[i];
        }
    }

    fn add2(&mut self, s: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[s].a[i] = self.mx[a].a[i] + self.mx[b].a[i];
        }
    }

    fn sub1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] -= self.mx[b].a[i];
        }
    }

    fn sub2(&mut self, d: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[d].a[i] = self.mx[a].a[i] - self.mx[b].a[i];
        }
    }

    fn mul2(&mut self, m: usize, a: usize, k: i32) {
        for i in 0..52 {
            self.mx[m].a[i] = self.mx[a].a[i] * k;
        }
    }

    fn cmp(&self, a: usize, b: usize) -> i32 {
        for i in 0..52 {
            let k = self.mx[a].a[i] - self.mx[b].a[i];
            if k != 0 {
                return k;
            }
        }
        0
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32, rcost: i32) {
        let f = self.hi[from];
        let t = self.hi[to];
        self.edges[from].push(Edge { to, cap, cost, rev: t });
        self.edges[to].push(Edge { to: from, cap: 0, cost: rcost, rev: f });
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, F: i32) -> usize {
        let mut ret = self.new_index();
        let mut t = self.new_index();
        let mut flow = F;

        while flow > 0 {
            for v in 0..self.V {
                self.clear(self.dist[v]);
            }
            self.clear(self.dist[S]);

            let mut update: bool;
            loop {
                update = false;
                for v in 0..self.V {
                    if self.cmp(self.dist[v], 1) == 0 {
                        continue;
                    }
                    for i in 0..self.hi[v] {
                        let e = &self.edges[v][i];
                        if e.cap <= 0 {
                            continue;
                        }
                        let nv = e.to;
                        self.add2(t, self.dist[v], e.cost);
                        if self.cmp(self.dist[nv], t) > 0 {
                            self.dist[nv] = t;
                            self.prevv[nv] = v;
                            self.preve[nv] = i;
                            update = true;
                        }
                    }
                }
                if !update {
                    break;
                }
            }

            let mut d = flow;
            let mut v = T;
            while v != S {
                let e = &self.edges[self.prevv[v]][self.preve[v]];
                if d > e.cap {
                    d = e.cap;
                }
                v = self.prevv[v];
            }
            flow -= d;

            self.mul2(t, self.dist[T], d);
            self.add1(ret, t);
            let mut v = T;
            while v != S {
                let e = &mut self.edges[self.prevv[v]][self.preve[v]];
                e.cap -= d;
                self.edges[v][e.rev].cap += d;
                v = self.prevv[v];
            }
        }
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut map = vec![String::new(); n];
    for i in 0..n {
        map[i] = lines.next().unwrap();
    }

    let mut flow_network = FlowNetwork::new(n * 2 + 2);
    let source = n * 2;
    let sink = source + 1;

    for i in 0..52 {
        flow_network.mx[1].a[i] = 60;
    }

    for i in 0..n {
        flow_network.add_edge(source, i, 1, flow_network.new_index() as i32, flow_network.new_index() as i32);
        for j in 0..n {
            let c = map[i].chars().nth(j).unwrap();
            let a = flow_network.new_index();
            let b = flow_network.new_index();
            if c.is_ascii_uppercase() {
                flow_network.mx[a].a[c as usize - 'A' as usize] = -1;
                flow_network.mx[b].a[c as usize - 'A' as usize] = 1;
            } else {
                flow_network.mx[a].a[26 + c as usize - 'a' as usize] = -1;
                flow_network.mx[b].a[26 + c as usize - 'a' as usize] = 1;
            }
            flow_network.add_edge(i, n + j, 1, a as i32, b as i32);
        }
        flow_network.add_edge(n + i, sink, 1, flow_network.new_index() as i32, flow_network.new_index() as i32);
    }

    let a = flow_network.min_cost_flow(source, sink, n as i32);

    for i in 0..26 {
        let mut b = -flow_network.mx[a].a[i];
        while b > 0 {
            print!("{}", (b'A' + i as u8) as char);
            b -= 1;
        }
    }
    for i in 0..26 {
        let mut b = -flow_network.mx[a].a[i + 26];
        while b > 0 {
            print!("{}", (b'a' + i as u8) as char);
            b -= 1;
        }
    }
    println!();
}
