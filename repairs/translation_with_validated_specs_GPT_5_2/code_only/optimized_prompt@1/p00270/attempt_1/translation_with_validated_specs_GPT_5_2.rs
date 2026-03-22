use std::io::{self, Read};

const QMAX: usize = 200000;
const INF: i32 = 0x1010_1010;

#[derive(Clone, Copy)]
struct Que {
    t: i32,
    s: i32,
}

struct MinHeap {
    a: Vec<Que>,
}
impl MinHeap {
    fn new() -> Self {
        Self { a: Vec::new() }
    }
    fn is_empty(&self) -> bool {
        self.a.is_empty()
    }
    fn push(&mut self, s: i32, t: i32) {
        let mut i = self.a.len();
        self.a.push(Que { s, t });
        while i > 0 {
            let p = (i - 1) >> 1;
            if self.a[p].t <= self.a[i].t {
                break;
            }
            self.a.swap(i, p);
            i = p;
        }
    }
    fn pop_min(&mut self) -> Que {
        let ret = self.a[0];
        let last = self.a.pop().unwrap();
        if !self.a.is_empty() {
            self.a[0] = last;
            self.min_heapify(0);
        }
        ret
    }
    fn min_heapify(&mut self, mut i: usize) {
        loop {
            let l = (i << 1) + 1;
            let r = l + 1;
            let mut min = i;
            if l < self.a.len() && self.a[l].t < self.a[min].t {
                min = l;
            }
            if r < self.a.len() && self.a[r].t < self.a[min].t {
                min = r;
            }
            if min == i {
                break;
            }
            self.a.swap(i, min);
            i = min;
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
    fn next_usize(&mut self) -> usize {
        // C in(): assumes digits; we skip non-digits to be robust with whitespace.
        while self.idx < self.buf.len() && (self.buf[self.idx] < b'0' || self.buf[self.idx] > b'9')
        {
            self.idx += 1;
        }
        let mut n: usize = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            self.idx += 1;
        }
        n
    }
}

fn dijkstra(start: usize, adj: &Vec<Vec<(usize, i32)>>, dist: &mut Vec<i32>) {
    let s = adj.len();
    dist.fill(INF);
    let mut heap = MinHeap::new();
    heap.push(start as i32, 0);
    dist[start] = 0;

    while !heap.is_empty() {
        let q = heap.pop_min();
        let u = q.s as usize;
        // Note: original C does not discard stale heap entries; we preserve that behavior.
        for &(v, w) in &adj[u] {
            let d = dist[u].wrapping_add(w); // preserve potential i32 wraparound
            if dist[v] > d {
                dist[v] = d;
                heap.push(v as i32, d);
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
    let s = adj.len();
    let mut mk = vec![0u8; s];
    let mut q = vec![0usize; s];
    let mut top = 0usize;
    let mut tail = 0usize;

    // Build reverse edges of shortest-path DAG reachable backwards from goal.
    q[tail] = goal;
    tail += 1;
    while top < tail {
        let u = q[top];
        top += 1;
        if mk[u] != 0 {
            continue;
        }
        for &(e, w) in &adj[u] {
            // Condition from C: dist[s] == dist[e] + w[s][i]
            if dist[u] == dist[e].wrapping_add(w) {
                q[tail] = e;
                tail += 1;
                to2[e].push(u); // edge e -> u in DAG
            }
        }
        mk[u] = 1;
    }

    // Kahn topological order from start over the constructed DAG.
    let mut deg = vec![0i32; s];
    for u in 0..s {
        for &v in &to2[u] {
            deg[v] += 1;
        }
    }

    top = 0;
    tail = 0;
    q[tail] = start;
    tail += 1;

    ord.clear();
    while top < tail {
        let u = q[top];
        top += 1;
        ord.push(u);
        for &v in &to2[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                q[tail] = v;
                tail += 1;
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();

    let s = sc.next_usize();
    let r = sc.next_usize();

    // First pass: count degrees to size adjacency lists.
    let mut hi = vec![0usize; s];
    let mut memo: Vec<(usize, usize, i32)> = Vec::with_capacity(r);
    for _ in 0..r {
        let u = sc.next_usize() - 1;
        let v = sc.next_usize() - 1;
        let w = sc.next_usize() as i32;
        hi[u] += 1;
        hi[v] += 1;
        memo.push((u, v, w));
    }

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

    // to2 is the shortest-path DAG edges (forward), built by tsort.
    let mut to2: Vec<Vec<usize>> = vec![Vec::new(); s];
    let mut ord: Vec<usize> = Vec::new();
    tsort(a, b, &adj, &dist, &mut to2, &mut ord);

    let mut out = String::new();

    let blocks = (qn + 63) >> 6;
    let mut dp = vec![0u64; s];
    for bi in 0..blocks {
        dp.fill(0);

        for j in 0..64usize {
            let idx = (bi << 6) + j;
            if idx < qn {
                dp[c[idx]] |= 1u64 << j;
            }
        }

        for &u in &ord {
            let val = dp[u];
            if val == 0 {
                continue;
            }
            for &v in &to2[u] {
                dp[v] |= val;
            }
        }

        for j in 0..64usize {
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
