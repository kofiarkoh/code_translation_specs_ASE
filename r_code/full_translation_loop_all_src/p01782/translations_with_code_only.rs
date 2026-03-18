use std::io::{self, BufRead};

#[derive(Clone)]
struct T {
    a: [i8; 52],
}

impl Default for T {
    fn default() -> Self {
        T { a: [0; 52] }
    }
}

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i32,
}

const MAX: usize = 105;
const MX_ZERO: usize = 0;
const MX_INF: usize = 1;

struct Graph {
    edges: [[Edge; MAX]; MAX],
    hi: [usize; MAX],
    mx: Vec<T>,
    sz: usize,
    dist: Vec<usize>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
    V: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: [[Edge { to: 0, rev: 0, cap: 0, cost: 0 }; MAX]; MAX],
            hi: [0; MAX],
            mx: vec![T::default(); 10000],
            sz: 0,
            dist: vec![0; MAX],
            prevv: vec![0; MAX],
            preve: vec![0; MAX],
            V: 0,
        }
    }

    fn mx_new(&mut self) -> usize {
        let idx = self.sz;
        self.sz += 1;
        idx
    }

    fn mx_clear(&mut self, a: usize) {
        self.mx[a] = T::default();
    }

    fn mx_add1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] = self.mx[a].a[i].wrapping_add(self.mx[b].a[i]);
        }
    }

    fn mx_add2(&mut self, s: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[s].a[i] = self.mx[a].a[i].wrapping_add(self.mx[b].a[i]);
        }
    }

    fn mx_sub1(&mut self, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[a].a[i] = self.mx[a].a[i].wrapping_sub(self.mx[b].a[i]);
        }
    }

    fn mx_sub2(&mut self, d: usize, a: usize, b: usize) {
        for i in 0..52 {
            self.mx[d].a[i] = self.mx[a].a[i].wrapping_sub(self.mx[b].a[i]);
        }
    }

    fn mx_mul2(&mut self, m: usize, a: usize, k: i32) {
        for i in 0..52 {
            self.mx[m].a[i] = (self.mx[a].a[i] as i32 * k) as i8;
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
        self.edges[from][f] = Edge { to, cap, cost, rev: t };
        self.edges[to][t] = Edge { to: from, cap: 0, cost: rcost, rev: f };
        self.hi[from] += 1;
        self.hi[to] += 1;
    }

    fn min_cost_flow(&mut self, S: usize, T: usize, F: &mut i32) -> usize {
        let mut ret = self.mx_new();
        let t = self.mx_new();

        while *F > 0 {
            for v in 0..self.V {
                self.mx_cpy(self.dist[v], MX_INF);
            }
            self.mx_clear(self.dist[S]);
            let mut update;

            loop {
                update = false;
                for v in 0..self.V {
                    if self.mx_cmp(self.dist[v], MX_INF) == 0 {
                        continue;
                    }
                    for i in 0..self.hi[v] {
                        let e = &self.edges[v][i];
                        if e.cap <= 0 {
                            continue;
                        }
                        let nv = e.to;
                        self.mx_add2(t, self.dist[v], e.cost);
                        if self.mx_cmp(self.dist[nv], t) > 0 {
                            self.mx_cpy(self.dist[nv], t);
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

            let mut d = *F;
            let mut v = T;
            while v != S {
                let e = &self.edges[self.prevv[v]][self.preve[v]];
                if d > e.cap {
                    d = e.cap;
                }
                v = self.prevv[v];
            }
            *F -= d;

            self.mx_mul2(t, self.dist[T], d);
            self.mx_add1(ret, t);
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

    let mut graph = Graph::new();
    let N: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut map = [[0; 53]; 53];
    for i in 0..N {
        let line = lines.next().unwrap();
        for j in 0..N {
            map[i][j] = line.as_bytes()[j] as char;
        }
    }

    for i in 0..52 {
        graph.mx[MX_INF].a[i] = 60;
    }
    graph.sz = 2;
    let source = N << 1;
    let sink = source + 1;
    graph.V = sink + 1;

    for i in 0..N {
        graph.add_edge(source, i, 1, graph.mx_new() as i32, graph.mx_new() as i32);
        for j in 0..N {
            let c = map[i][j];
            let a = graph.mx_new();
            let b = graph.mx_new();
            if c < 'a' {
                graph.mx[a].a[c as usize - 'A' as usize] = -1;
                graph.mx[b].a[c as usize - 'A' as usize] = 1;
            } else {
                graph.mx[a].a[26 + c as usize - 'a' as usize] = -1;
                graph.mx[b].a[26 + c as usize - 'a' as usize] = 1;
            }
            graph.add_edge(i, N + j, 1, a as i32, b as i32);
        }
        graph.add_edge(N + i, sink, 1, graph.mx_new() as i32, graph.mx_new() as i32);
    }

    let mut a = graph.min_cost_flow(source, sink, &mut N as *mut _ as *mut i32);

    for i in 0..26 {
        let mut b = -graph.mx[a].a[i];
        while b > 0 {
            print!("{}", 'A' as u8 as char);
            b -= 1;
        }
    }
    for i in 0..26 {
        let mut b = -graph.mx[a].a[i + 26];
        while b > 0 {
            print!("{}", 'a' as u8 as char);
            b -= 1;
        }
    }
    println!();
}
