use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    c: i32,
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn skip_ws(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }
    fn next_i32(&mut self) -> i32 {
        self.skip_ws();
        let mut sign = 1i32;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        sign * n
    }
    fn next_usize(&mut self) -> usize {
        self.next_i32() as usize
    }
    fn next_f64(&mut self) -> f64 {
        self.skip_ws();
        let mut sign = 1.0f64;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            sign = -1.0;
            self.idx += 1;
        }
        let mut int_part: i64 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            int_part = int_part * 10 + (c - b'0') as i64;
            self.idx += 1;
        }
        let mut x = int_part as f64;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'.' {
            self.idx += 1;
            let mut place = 0.1f64;
            while self.idx < self.buf.len() {
                let c = self.buf[self.idx];
                if !c.is_ascii_digit() {
                    break;
                }
                x += (c - b'0') as f64 * place;
                place *= 0.1;
                self.idx += 1;
            }
        }
        sign * x
    }
}

fn main() {
    // Read input
    let mut sc = Scanner::new();
    let p_base: f64 = sc.next_f64();
    let n: usize = sc.next_usize();

    // Build adjacency list for an undirected tree.
    // We first count degrees, then allocate exact-sized vectors (like the C code).
    let mut deg = vec![0usize; n + 1];
    let mut edges_raw: Vec<(usize, usize, i32)> = Vec::with_capacity(n.saturating_sub(1));
    for _ in 1..n {
        let x = sc.next_usize();
        let y = sc.next_usize();
        let c = sc.next_i32();
        edges_raw.push((x, y, c));
        if x <= n {
            deg[x] += 1;
        }
        if y <= n {
            deg[y] += 1;
        }
    }

    let mut adj: Vec<Vec<Edge>> = (0..=n).map(|i| Vec::with_capacity(deg[i])).collect();
    for (x, y, c) in edges_raw {
        if x <= n && y <= n {
            adj[x].push(Edge { to: y, c });
            adj[y].push(Edge { to: x, c });
        }
    }

    // Iterative DFS from root 1 to establish depth and parent-edge weight (cost).
    let mut vis = vec![false; n + 1];
    let mut depth = vec![0usize; n + 1];
    let mut cost = vec![0i32; n + 1]; // parent-edge weight; root stays 0
    let mut depth_max: usize = 0;

    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(n);
    if n >= 1 {
        stack.push((1, 0));
    }
    while let Some((node, d)) = stack.pop() {
        if node > n || vis[node] {
            continue;
        }
        vis[node] = true;
        depth[node] = d;
        if d > depth_max {
            depth_max = d;
        }
        for &e in &adj[node] {
            if e.to <= n && !vis[e.to] {
                cost[e.to] = e.c;
                stack.push((e.to, d + 1));
            }
        }
    }

    // Precompute scaling factors s(d) = p_base^d up to max depth.
    let mut p = vec![0f64; depth_max + 1];
    if !p.is_empty() {
        p[0] = 1.0;
        for i in 1..=depth_max {
            p[i] = p[i - 1] * p_base;
        }
    }

    // Compute:
    // A = sum_v cost[v] * s(depth[v])
    // B = sum_v s(depth[v])
    // Output A * B
    //
    // Note: The original C initializes b=1 and then adds p[depth[i]] for i=1..N,
    // effectively counting the root twice. That conflicts with the SPEC, which
    // defines B as sum over all nodes exactly once. We follow the SPEC.
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    for v in 1..=n {
        let d = depth[v];
        let sd = p[d];
        a += (cost[v] as f64) * sd;
        b += sd;
    }

    println!("{:.10}", a * b);
}
