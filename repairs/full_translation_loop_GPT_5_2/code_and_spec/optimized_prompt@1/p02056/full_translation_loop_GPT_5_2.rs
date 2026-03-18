use std::cmp::Ordering;
use std::io::{self, Read};

type I32 = i32;
type I64 = i64;

#[derive(Clone, Copy)]
struct Edge {
    to: I32,
    next: I32,
    cost: I32,
}

struct Graph {
    edges: Vec<Edge>,
    start: Vec<I32>,
}

impl Graph {
    fn new(vertex_num: usize) -> Self {
        Self {
            edges: Vec::new(),
            start: vec![-1; vertex_num],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: I32) {
        let idx = self.edges.len() as I32;
        let next = self.start[from];
        self.edges.push(Edge {
            to: to as I32,
            next,
            cost,
        });
        self.start[from] = idx;
    }
}

#[derive(Clone, Copy)]
struct Node {
    v: usize,
    d: I64,
}

// Min-heap by distance (like cmp_node in C).
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior in BinaryHeap.
        other.d.cmp(&self.d)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d && self.v == other.v
    }
}
impl Eq for Node {}

fn main() {
    // Input parsing: behavior undefined on malformed input (matches spec/C).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse::<I32>().unwrap() as usize;
    let m: usize = it.next().unwrap().parse::<I32>().unwrap() as usize;
    let k: usize = it.next().unwrap().parse::<I32>().unwrap() as usize;

    // C reads 3*n integers into a single block, then splits into p,c,t.
    let mut p = vec![0i32; n];
    let mut c = vec![0i32; n];
    let mut t = vec![0i32; n];
    for i in 0..n {
        p[i] = it.next().unwrap().parse::<I32>().unwrap();
    }
    for i in 0..n {
        c[i] = it.next().unwrap().parse::<I32>().unwrap();
    }
    for i in 0..n {
        t[i] = it.next().unwrap().parse::<I32>().unwrap();
    }

    let sup: I32 = 1_000_000_000 + 1;

    // Graph has 2*n vertices.
    let mut g = Graph::new(2 * n);

    // Add edges i -> i+n with cost sup - p[i]
    for i in 0..n {
        g.add_edge(i, i + n, sup - p[i]);
    }

    // Read m undirected edges, add to both layers.
    for _ in 0..m {
        let mut a: I32 = it.next().unwrap().parse().unwrap();
        let mut b: I32 = it.next().unwrap().parse().unwrap();
        let d: I32 = it.next().unwrap().parse().unwrap();
        a -= 1;
        b -= 1;
        let au = a as usize;
        let bu = b as usize;

        g.add_edge(au, bu, d);
        g.add_edge(bu, au, d);

        g.add_edge(au + n, bu + n, d);
        g.add_edge(bu + n, au + n, d);
    }

    // Dijkstra from vertex 0.
    let inf: I64 = 1_000_000_001i64 * 2i64 * (n as I64);
    let mut dp = vec![inf; 2 * n];
    dp[0] = 0;

    let mut heap = std::collections::BinaryHeap::<Node>::new();
    heap.push(Node { v: 0, d: 0 });

    while let Some(cur) = heap.pop() {
        if cur.d > dp[cur.v] {
            continue;
        }
        let mut eidx = g.start[cur.v];
        while eidx != -1 {
            let e = g.edges[eidx as usize];
            let u = e.to as usize;
            let nd = cur.d + e.cost as I64;
            if nd < dp[u] {
                dp[u] = nd;
                heap.push(Node { v: u, d: nd });
            }
            eidx = e.next;
        }
    }

    // max[1..=k], initialized to 2 - inf (as in C).
    let mut maxv = vec![0i64; k + 1];
    let init = 2i64 - inf;
    for i in 1..=k {
        maxv[i] = init;
    }

    for i in 0..n {
        // v = (sup - dp[i+n]) - dp[i] + t[i]
        let v = (sup as I64 - dp[i + n]) - dp[i] + t[i] as I64;
        let ci = c[i] as usize;
        if ci <= k {
            if v > maxv[ci] {
                maxv[ci] = v;
            }
        }
        // Note: C assumes 1 <= c[i] <= k; if not, behavior is undefined.
        // We keep defined Rust behavior by ignoring out-of-range indices.
    }

    let mut out = String::new();
    for i in 1..=k {
        out.push_str(&format!("{}\n", maxv[i]));
    }
    print!("{out}");
}
