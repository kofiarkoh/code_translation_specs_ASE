use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cost: i64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    d: i64,
    v: usize,
}

// Min-heap by distance (BinaryHeap is max-heap by default).
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering on distance; tie-break on vertex for determinism.
        other
            .d
            .cmp(&self.d)
            .then_with(|| other.v.cmp(&self.v))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Read all input (matches C scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    // Read 3n integers: p[0..n), c[0..n), t[0..n)
    let mut p = vec![0i64; n];
    let mut c = vec![0usize; n];
    let mut t = vec![0i64; n];
    for i in 0..n {
        p[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    for i in 0..n {
        c[i] = it.next().unwrap().parse::<usize>().unwrap();
    }
    for i in 0..n {
        t[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    // Spec: S is a fixed constant > 1e9. C uses 1_000_000_000 + 1.
    let s: i64 = 1_000_000_001;

    // Build directed graph with 2n vertices.
    let vn = 2 * n;
    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); vn];

    // Cross-layer edges i -> i+n with weight (S - p[i]).
    for i in 0..n {
        g[i].push(Edge {
            to: i + n,
            cost: s - p[i],
        });
    }

    // Undirected edges in both layers become directed edges both ways.
    for _ in 0..m {
        let mut a: usize = it.next().unwrap().parse().unwrap();
        let mut b: usize = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        a -= 1;
        b -= 1;

        g[a].push(Edge { to: b, cost: d });
        g[b].push(Edge { to: a, cost: d });

        let a2 = a + n;
        let b2 = b + n;
        g[a2].push(Edge { to: b2, cost: d });
        g[b2].push(Edge { to: a2, cost: d });
    }

    // Dijkstra from source 0.
    // C initializes dp[0]=0 (via calloc) and dp[1..]= (1_000_000_001 * 2 * n).
    let inf: i64 = 1_000_000_001i64 * 2i64 * (n as i64);
    let mut dist = vec![inf; vn];
    dist[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State { d: 0, v: 0 });

    while let Some(State { d, v }) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &Edge { to, cost } in &g[v] {
            let nd = d + cost;
            if nd < dist[to] {
                dist[to] = nd;
                heap.push(State { d: nd, v: to });
            }
        }
    }

    // Per-category maxima. C uses sentinel: 2 - (1_000_000_001 * 2 * n).
    let sentinel: i64 = 2 - inf;
    let mut maxv = vec![sentinel; k + 1]; // 1..=k used

    for i in 0..n {
        // score(i) = (S - dist(i+n)) - dist(i) + t[i]
        let score = (s - dist[i + n]) - dist[i] + t[i];
        let cat = c[i]; // precondition: 1..=k
        if score > maxv[cat] {
            maxv[cat] = score;
        }
    }

    // Output exactly k lines.
    let mut out = String::new();
    for j in 1..=k {
        out.push_str(&format!("{}\n", maxv[j]));
    }
    print!("{out}");
}
