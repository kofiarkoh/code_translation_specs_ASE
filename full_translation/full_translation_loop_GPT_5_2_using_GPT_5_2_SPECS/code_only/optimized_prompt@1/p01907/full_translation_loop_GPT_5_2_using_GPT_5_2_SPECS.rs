use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    c: i32,
}

#[derive(Clone, Copy)]
struct QItem {
    n: usize,
    d: i32,
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

    fn gc(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            0
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }

    // Emulates the C in(): reads a non-negative integer, assuming input is well-formed.
    fn in_u32(&mut self) -> u32 {
        let mut n: u32 = 0;
        let mut c = self.gc();
        // C code does not skip whitespace; it assumes the first char is a digit.
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            c = self.gc();
            if c < b'0' {
                break;
            }
        }
        n
    }

    // Emulates the C dbl(): reads a real number with optional leading '-',
    // integer part (at least one digit), and optional fractional part.
    fn dbl(&mut self) -> f64 {
        let mut minus = false;
        let mut n: u64 = 0;
        let mut c = self.gc();
        if c == b'-' {
            minus = true;
            c = self.gc();
        }
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u64);
            c = self.gc();
            if c < b'0' {
                break;
            }
        }

        let mut x: f64;
        if c == b'.' {
            let mut frac = 0.0f64;
            let mut y = 1.0f64;
            c = self.gc();
            while c >= b'0' {
                y *= 0.1;
                frac += y * ((c & 0x0f) as f64);
                c = self.gc();
            }
            x = (n as f64) + frac;
        } else {
            x = n as f64;
        }
        if minus {
            x = -x;
        }
        x
    }
}

fn main() {
    const MAX: usize = 100005;

    let mut sc = Scanner::new();

    let p_val = sc.dbl();
    let n = sc.in_u32() as usize;

    // First pass: read edges, count degrees.
    let mut hi = vec![0usize; MAX];
    let mut memo: Vec<(usize, usize, i32)> = Vec::with_capacity(3 * n);
    for _ in 1..n {
        let x = sc.in_u32() as usize;
        let y = sc.in_u32() as usize;
        let c = sc.in_u32() as i32;
        memo.push((x, y, c));
        hi[x] += 1;
        hi[y] += 1;
    }

    // Allocate adjacency lists with exact sizes, then fill.
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); MAX];
    for i in 1..=n {
        if hi[i] != 0 {
            adj[i] = Vec::with_capacity(hi[i]);
        }
    }
    // Reset hi[1..=n] to 0 like memset(hi+1, 0, sizeof(int)*N)
    for i in 1..=n {
        hi[i] = 0;
    }

    for (x, y, c) in memo {
        adj[x].push(Edge { to: y, c });
        hi[x] += 1;
        adj[y].push(Edge { to: x, c });
        hi[y] += 1;
    }

    // Iterative DFS as in C code.
    let mut cost = vec![0i32; MAX];
    let mut depth = vec![0i32; MAX];
    let mut depth_max: i32 = 0;
    let mut vis = vec![0u8; MAX];
    let mut q = vec![QItem { n: 0, d: 0 }; MAX];
    let mut top: usize = 0;

    q[0] = QItem { n: 1, d: 0 };
    top = 1;
    while top != 0 {
        top -= 1;
        let node = q[top].n;
        let d = q[top].d;

        if vis[node] != 0 {
            continue;
        }
        vis[node] = 1;

        depth[node] = d;
        if d > depth_max {
            depth_max = d;
        }
        for e in adj[node].iter() {
            let to = e.to;
            if vis[to] == 0 {
                cost[to] = e.c;
                q[top] = QItem { n: to, d: d + 1 };
                top += 1;
            }
        }
    }

    // p[i] = P^i up to depth_max
    let mut p = vec![0f64; MAX];
    p[0] = 1.0;
    for i in 1..=(depth_max as usize) {
        p[i] = p[i - 1] * p_val;
    }

    let mut a = 0.0f64;
    let mut b = 1.0f64;
    for i in 1..=n {
        let di = depth[i] as usize;
        a += (cost[i] as f64) * p[di];
        b += p[di];
    }

    // Match C printf("%.10lf\n", a*b);
    println!("{:.10}", a * b);
}
