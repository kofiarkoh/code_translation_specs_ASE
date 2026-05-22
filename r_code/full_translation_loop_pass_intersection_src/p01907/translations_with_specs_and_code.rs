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

    fn skip_whitespace(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }

    fn next_i32(&mut self) -> i32 {
        self.skip_whitespace();
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
            n = n.saturating_mul(10).saturating_add((c - b'0') as i32);
            self.idx += 1;
        }
        sign * n
    }

    fn next_usize(&mut self) -> usize {
        self.next_i32() as usize
    }

    fn next_f64(&mut self) -> f64 {
        self.skip_whitespace();
        let mut sign = 1.0f64;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            sign = -1.0;
            self.idx += 1;
        }

        // integer part
        let mut int_part: u64 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            int_part = int_part * 10 + (c - b'0') as u64;
            self.idx += 1;
        }

        let mut x = int_part as f64;

        // fractional part
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
    let p_in: f64 = sc.next_f64();
    let n: usize = sc.next_usize();

    // Build adjacency list (undirected). Spec allows non-tree; we still traverse from node 1.
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n + 1];
    if n >= 2 {
        for _ in 1..n {
            let x = sc.next_usize();
            let y = sc.next_usize();
            let c = sc.next_i32();
            if x <= n && y <= n {
                adj[x].push(Edge { to: y, c });
                adj[y].push(Edge { to: x, c });
            } else {
                // Spec precondition says indices are within [1,N]; no explicit error handling.
                // If violated, we ignore out-of-range edges to avoid panics.
            }
        }
    }

    // Iterative DFS from node 1, assigning depth and parent-edge weight on first discovery.
    let mut vis = vec![false; n + 1];
    let mut depth = vec![0usize; n + 1];
    let mut cost = vec![0i32; n + 1];
    let mut depth_max: usize = 0;

    if n >= 1 {
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(n.max(1));
        stack.push((1, 0));
        while let Some((node, d)) = stack.pop() {
            if node == 0 || node > n {
                continue;
            }
            if vis[node] {
                continue;
            }
            vis[node] = true;
            depth[node] = d;
            if d > depth_max {
                depth_max = d;
            }
            for &e in &adj[node] {
                let to = e.to;
                if to >= 1 && to <= n && !vis[to] {
                    // Record the edge weight by which `to` is first reached.
                    // If `to` is pushed multiple times before being visited, the last push wins,
                    // matching the C code's behavior.
                    cost[to] = e.c;
                    stack.push((to, d + 1));
                }
            }
        }
    }

    // Precompute powers P^d up to max depth reached.
    let mut powp = vec![0.0f64; depth_max + 1];
    if !powp.is_empty() {
        powp[0] = 1.0;
        for i in 1..=depth_max {
            powp[i] = powp[i - 1] * p_in;
        }
    }

    // Accumulate:
    // a = sum cost[v] * P^depth[v]
    // b = 1 + sum P^depth[v]
    let mut a: f64 = 0.0;
    let mut b: f64 = 1.0;
    for v in 1..=n {
        let d = depth[v];
        let pd = if d <= depth_max { powp[d] } else { 0.0 };
        a += (cost[v] as f64) * pd;
        b += pd;
    }

    let ans = a * b;
    print!("{:.10}\n", ans);
}
