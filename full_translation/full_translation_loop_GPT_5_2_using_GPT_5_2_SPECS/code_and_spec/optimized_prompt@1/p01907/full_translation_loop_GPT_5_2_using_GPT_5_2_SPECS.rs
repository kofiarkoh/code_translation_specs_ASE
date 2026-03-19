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
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { buf: s.into_bytes(), idx: 0 }
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
    let mut sc = Scanner::new();
    let p_base = sc.next_f64();
    let n = sc.next_usize();

    let mut deg = vec![0usize; n + 1];
    let mut raw: Vec<(usize, usize, i32)> = Vec::with_capacity(n.saturating_sub(1));
    for _ in 1..n {
        let x = sc.next_usize();
        let y = sc.next_usize();
        let c = sc.next_i32();
        raw.push((x, y, c));
        deg[x] += 1;
        deg[y] += 1;
    }

    let mut adj: Vec<Vec<Edge>> = (0..=n).map(|i| Vec::with_capacity(deg[i])).collect();
    for (x, y, c) in raw {
        adj[x].push(Edge { to: y, c });
        adj[y].push(Edge { to: x, c });
    }

    // DFS (iterative) from root 1, matching the C behavior.
    let mut vis = vec![false; n + 1];
    let mut depth = vec![0usize; n + 1];
    let mut cost = vec![0i32; n + 1]; // parent-edge weight; root remains 0
    let mut depth_max = 0usize;

    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(n);
    if n >= 1 {
        stack.push((1, 0));
    }
    while let Some((node, d)) = stack.pop() {
        if vis[node] {
            continue;
        }
        vis[node] = true;
        depth[node] = d;
        depth_max = depth_max.max(d);

        for &e in &adj[node] {
            if !vis[e.to] {
                cost[e.to] = e.c;
                stack.push((e.to, d + 1));
            }
        }
    }

    // Precompute powers up to max depth.
    let mut p = vec![0f64; depth_max + 1];
    p[0] = 1.0;
    for i in 1..=depth_max {
        p[i] = p[i - 1] * p_base;
    }

    // Compute sums.
    let mut a = 0.0f64;
    // NOTE: The provided C code initializes b=1 and then adds p[depth[i]] for i=1..N,
    // which counts the root's scaling factor twice. The sample output matches the C code,
    // so we follow that behavior here (even though it conflicts with the written SPEC).
    let mut b = 1.0f64;

    for v in 1..=n {
        let sd = p[depth[v]];
        a += (cost[v] as f64) * sd;
        b += sd;
    }

    print!("{:.10}\n", a * b);
}
