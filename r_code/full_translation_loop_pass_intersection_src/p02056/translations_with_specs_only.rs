use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const S: i64 = 1_000_000_000_000_000_000; // fixed constant > 1e9
const NEG_SENTINEL: i64 = -1_000_000_000_000_000_000; // large-magnitude negative 64-bit integer

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    w: i64,
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    // Per-vertex arrays
    // p[i]: used in cross-layer edge weight (S - p[i])
    // c[i]: category index in 1..k (otherwise behavior undefined per spec)
    // t[i]: additive term in final score
    let mut p = vec![0i64; n];
    let mut c = vec![0usize; n];
    let mut t = vec![0i64; n];

    for i in 0..n {
        p[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 0..n {
        c[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 0..n {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // Build graph with 2n vertices
    let nn = 2 * n;
    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); nn];

    // Add undirected edges in both layers as directed edges
    for _ in 0..m {
        let a1: usize = it.next().unwrap().parse().unwrap();
        let b1: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        let a = a1 - 1;
        let b = b1 - 1;

        g[a].push(Edge { to: b, w });
        g[b].push(Edge { to: a, w });

        g[a + n].push(Edge { to: b + n, w });
        g[b + n].push(Edge { to: a + n, w });
    }

    // Add cross-layer edges i -> i+n with weight (S - p[i])
    for i in 0..n {
        let w = S - p[i];
        g[i].push(Edge { to: i + n, w });
    }

    // Dijkstra from source 0
    let inf: i64 = i64::MAX / 4;
    let mut dist = vec![inf; nn];
    dist[0] = 0;

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), 0));

    while let Some((Reverse(d), v)) = pq.pop() {
        // Ignore stale candidates
        if d != dist[v] {
            continue;
        }
        for &Edge { to, w } in &g[v] {
            // Spec assumes no overflow; we use normal i64 arithmetic.
            let nd = d + w;
            if nd < dist[to] {
                dist[to] = nd;
                pq.push((Reverse(nd), to));
            }
        }
    }

    // Aggregate per-category maxima (1-based categories)
    let mut best = vec![NEG_SENTINEL; k + 1]; // index 0 unused

    for i in 0..n {
        // score(i) = (S - dist(i+n)) - dist(i) + t[i]
        let score = (S - dist[i + n]) - dist[i] + t[i];
        let cat = c[i]; // assumed 1..k per spec; otherwise undefined behavior
        if score > best[cat] {
            best[cat] = score;
        }
    }

    // Output exactly k lines
    let mut out = String::new();
    for j in 1..=k {
        out.push_str(&format!("{}\n", best[j]));
    }
    print!("{out}");
}
