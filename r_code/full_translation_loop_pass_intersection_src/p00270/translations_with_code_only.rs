use std::io::{self, Read};

const QMAX: usize = 200000;
const INF: i32 = 0x10101010;

#[derive(Clone, Copy, Default)]
struct QueItem {
    t: i32,
    s: usize,
}

struct MinHeap {
    a: Vec<QueItem>,
}
impl MinHeap {
    fn new() -> Self {
        Self { a: Vec::new() }
    }
    fn clear(&mut self) {
        self.a.clear();
    }
    fn is_empty(&self) -> bool {
        self.a.is_empty()
    }
    fn push(&mut self, s: usize, t: i32) {
        // C code uses a fixed array of size QMAX+2; we emulate capacity but allow growth safely.
        if self.a.len() < QMAX + 2 {
            // ok
        }
        let mut i = self.a.len();
        self.a.push(QueItem { s, t });
        while i > 0 {
            let p = (i - 1) >> 1;
            if self.a[p].t <= self.a[i].t {
                break;
            }
            self.a.swap(i, p);
            i = p;
        }
    }
    fn pop_min(&mut self) -> Option<QueItem> {
        if self.a.is_empty() {
            return None;
        }
        let min = self.a[0];
        let last = self.a.pop().unwrap();
        if !self.a.is_empty() {
            self.a[0] = last;
            self.min_heapify(0);
        }
        Some(min)
    }
    fn min_heapify(&mut self, mut i: usize) {
        loop {
            let l = (i << 1) + 1;
            let r = l + 1;
            let mut m = i;
            if l < self.a.len() && self.a[l].t < self.a[m].t {
                m = l;
            }
            if r < self.a.len() && self.a[r].t < self.a[m].t {
                m = r;
            }
            if m == i {
                break;
            }
            self.a.swap(i, m);
            i = m;
        }
    }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }
    fn next_u32(&mut self) -> u32 {
        // The C in() assumes digits start immediately (no skipping) and stops at first non-digit.
        // AOJ inputs are well-formed; we still skip non-digits to be robust.
        while self.idx < self.buf.len() && (self.buf[self.idx] < b'0' || self.buf[self.idx] > b'9')
        {
            self.idx += 1;
        }
        let mut n: u32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as u32;
            self.idx += 1;
        }
        n
    }
    fn next_usize(&mut self) -> usize {
        self.next_u32() as usize
    }
    fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }
}

fn dijkstra(start: usize, adj: &Vec<Vec<(usize, i32)>>, dist: &mut Vec<i32>) {
    // memset(dist, INF, S<<2) in C sets each byte to 0x10, yielding 0x10101010.
    for d in dist.iter_mut() {
        *d = INF;
    }
    let mut heap = MinHeap::new();
    heap.clear();
    dist[start] = 0;
    heap.push(start, 0);

    while !heap.is_empty() {
        let item = heap.pop_min().unwrap();
        let s = item.s;
        // Note: C code does not discard stale heap entries; we preserve that behavior.
        for &(e, w) in &adj[s] {
            // C uses int arithmetic; overflow is UB in C, but likely not reached.
            // Use wrapping_add to emulate typical 2's complement wrap if it happens.
            let nd = dist[s].wrapping_add(w);
            if dist[e] > nd {
                dist[e] = nd;
                heap.push(e, nd);
            }
        }
    }
}

fn tsort(
    start: usize,
    goal: usize,
    adj: &Vec<Vec<(usize, i32)>>,
    dist: &Vec<i32>,
    to2: &mut Vec<Vec<usize>>,
    ord: &mut Vec<usize>,
) {
    let s_len = adj.len();
    let mut mk = vec![0u8; s_len];
    let mut q = vec![0usize; s_len + 5];
    let mut top = 0usize;
    let mut tail = 0usize;

    // Build reverse edges of shortest-path DAG: e -> s when s can be reached from e on a shortest path.
    q[0] = goal;
    top = 0;
    tail = 1;
    while top < tail {
        let s = q[top];
        top += 1;
        if mk[s] != 0 {
            continue;
        }
        for &(e, w) in &adj[s] {
            if dist[s] == dist[e].wrapping_add(w) {
                q[tail] = e;
                tail += 1;
                to2[e].push(s);
            }
        }
        mk[s] = 1;
    }

    // Kahn topological order from start over this DAG.
    let mut deg = vec![0i32; s_len];
    for s in 0..s_len {
        for &v in &to2[s] {
            deg[v] += 1;
        }
    }

    ord.clear();
    q[0] = start;
    top = 0;
    tail = 1;
    while top < tail {
        let s = q[top];
        top += 1;
        ord.push(s);
        for &e in &to2[s] {
            deg[e] -= 1;
            if deg[e] == 0 {
                q[tail] = e;
                tail += 1;
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();

    let s = sc.next_usize();
    let r = sc.next_usize();

    let mut hi = vec![0usize; s];
    let mut memo: Vec<(usize, usize, i32)> = Vec::with_capacity(r);

    for _ in 0..r {
        let u = sc.next_usize() - 1;
        let v = sc.next_usize() - 1;
        let w = sc.next_i32();
        hi[u] += 1;
        hi[v] += 1;
        memo.push((u, v, w));
    }

    // Build undirected adjacency lists with exact sizes.
    let mut adj: Vec<Vec<(usize, i32)>> = (0..s).map(|i| Vec::with_capacity(hi[i])).collect();
    for (u, v, w) in memo {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let a = sc.next_usize() - 1;
    let b = sc.next_usize() - 1;
    let qn = sc.next_usize();

    let mut c = vec![0usize; qn];
    let mut d = vec![0usize; qn];
    for i in 0..qn {
        c[i] = sc.next_usize() - 1;
        d[i] = sc.next_usize() - 1;
    }

    let mut dist = vec![INF; s];
    dijkstra(a, &adj, &mut dist);

    let mut to2: Vec<Vec<usize>> = vec![Vec::new(); s];
    let mut ord: Vec<usize> = Vec::new();
    tsort(a, b, &adj, &dist, &mut to2, &mut ord);

    let mut out = String::new();
    let blocks = (qn + 63) >> 6;
    let mut dp = vec![0u64; s];

    for bi in 0..blocks {
        // memset(dp, 0, sizeof(dp))
        for x in dp.iter_mut() {
            *x = 0;
        }

        for j in 0..64 {
            let idx = (bi << 6) + j;
            if idx < qn {
                dp[c[idx]] |= 1u64 << j;
            }
        }

        for &u in &ord {
            let val = dp[u];
            for &v in &to2[u] {
                dp[v] |= val;
            }
        }

        for j in 0..64 {
            let idx = (bi << 6) + j;
            if idx < qn {
                if (dp[d[idx]] & (1u64 << j)) != 0 {
                    out.push_str("Yes\n");
                } else {
                    out.push_str("No\n");
                }
            }
        }
    }

    print!("{out}");
}
