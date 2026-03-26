use std::io::{self, BufRead};

#[derive(Default, Clone)]
struct T {
    a: [i8; 52],
}

#[derive(Default, Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

struct FlowNetwork {
    mx: Vec<T>,
    sz: usize,
    edges: Vec<Vec<Edge>>,
    hi: Vec<usize>,
    V: usize,
}

impl FlowNetwork {
    fn new() -> Self {
        Self {
            mx: vec![T::default(); 10000],
            sz: 0,
            edges: vec![vec![]; 105],
            hi: vec![0; 105],
            V: 0,
        }
    }

    fn mx_new(&mut self) -> usize {
        let index = self.sz;
        self.sz += 1;
        index
    }

    fn mx_clear(&mut self, a: usize) {
        self.mx[a] = T::default();
    }

    fn mx_add1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] += self.mx[b].a[i];
        }
    }

    fn mx_add2(&mut self, s: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[s].a[i] = self.mx[a].a[i] + self.mx[b].a[i];
        }
    }

    fn mx_sub1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] -= self.mx[b].a[i];
        }
    }

    fn mx_sub2(&mut self, d: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[d].a[i] = self.mx[a].a[i] - self.mx[b].a[i];
        }
    }

    fn mx_mul2(&mut self, m: usize, a: usize, k: i8) {
        for i in 0..52 {
            self.mx[m].a[i] = self.mx[a].a[i] * k;
        }
    }

    fn mx_cpy(&mut self, a: usize, b: usize) {
        self.mx[a] = self.mx[b].clone();
    }

    fn mx_cmp(&self, a: usize, b: usize) -> i32 {
        for i in 0..52 {
            let k = self.mx[a].a[i] - self.mx[b].a[i];
            if k != 0 {
                return k as i32;
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
        let mut dist = vec![self.mx_new(); self.V];
        let mut prevv = vec![0; self.V];
        let mut preve = vec![0; self.V];
        let mut ret = self.mx_new();
        let mut flow = F;

        while flow > 0 {
            for v in 0..self.V {
                self.mx_cpy(dist[v], self.mx_new());
            }
            self.mx_clear(dist[S]);

            let mut update: bool;
            loop {
                update = false;
                for v in 0..self.V {
                    if self.mx_cmp(dist[v], self.mx_new()) == 0 {
                        continue;
                    }
                    for i in 0..self.hi[v] {
                        let e = &self.edges[v][i];
                        if e.cap <= 0 {
                            continue;
                        }
                        let nv = e.to;
                        let temp_index = self.mx_new();
                        self.mx_add2(temp_index, dist[v], e.cost);
                        if self.mx_cmp(dist[nv], temp_index) > 0 {
                            self.mx_cpy(dist[nv], temp_index);
                            prevv[nv] = v;
                            preve[nv] = i;
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
                let e = &self.edges[prevv[v]][preve[v]];
                if d > e.cap {
                    d = e.cap;
                }
                v = prevv[v];
            }
            flow -= d;

            let temp_index = self.mx_new();
            self.mx_mul2(temp_index, dist[T], d as i8);
            self.mx_add1(ret, temp_index);
            let mut v = T;
            while v != S {
                let e = &mut self.edges[prevv[v]][preve[v]];
                e.cap -= d;
                self.edges[v][e.rev].cap += d;
                v = prevv[v];
            }
        }
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut map = vec![String::new(); n];

    for i in 0..n {
        map[i] = lines.next().unwrap().unwrap();
    }

    let mut flow_network = FlowNetwork::new();
    let source = n * 2;
    let sink = source + 1;
    flow_network.V = sink + 1;

    for i in 0..52 {
        flow_network.mx[flow_network.mx_new()].a[i] = 60;
    }

    for i in 0..n {
        flow_network.add_edge(source, i, 1, flow_network.mx_new() as i32, flow_network.mx_new() as i32);
        for j in 0..n {
            let c = map[i].chars().nth(j).unwrap();
            let a = flow_network.mx_new();
            let b = flow_network.mx_new();
            if c.is_ascii_uppercase() {
                flow_network.mx[a].a[c as usize - 'A' as usize] = -1;
                flow_network.mx[b].a[c as usize - 'A' as usize] = 1;
            } else {
                flow_network.mx[a].a[26 + c as usize - 'a' as usize] = -1;
                flow_network.mx[b].a[26 + c as usize - 'a' as usize] = 1;
            }
            flow_network.add_edge(i, n + j, 1, a as i32, b as i32);
        }
        flow_network.add_edge(n + i, sink, 1, flow_network.mx_new() as i32, flow_network.mx_new() as i32);
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
