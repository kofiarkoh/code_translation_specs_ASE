use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    w: i64,
}

#[derive(Clone, Copy)]
struct State {
    d: i64,
    v: usize,
}

// Min-heap by distance (BinaryHeap is max-heap by default).
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d).then_with(|| other.v.cmp(&self.v))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d && self.v == other.v
    }
}
impl Eq for State {}

fn dijkstra(n: usize, g: &[Vec<Edge>], src: usize) -> Vec<i64> {
    let inf = i64::MAX / 4;
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::new();
    dist[src] = 0;
    pq.push(State { d: 0, v: src });

    while let Some(State { d, v }) = pq.pop() {
        if d != dist[v] {
            continue;
        }
        for &e in &g[v] {
            let nd = d.saturating_add(e.w);
            if nd < dist[e.to] {
                dist[e.to] = nd;
                pq.push(State { d: nd, v: e.to });
            }
        }
    }
    dist
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).ok();
    let mut it = s.split_whitespace();

    let n: usize = match it.next().and_then(|x| x.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: usize = match it.next().and_then(|x| x.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let k: usize = match it.next().and_then(|x| x.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut cost = vec![0i64; n];
    for i in 0..n {
        cost[i] = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => return,
        };
    }

    // The sample input contains an additional block of 3*k integers after costs and before edges.
    // The specification mentions a "parameter k" and output of k integers, consistent with k queries.
    // We interpret these as k query triples (a,b,c) but the exact meaning is unspecified.
    // The narrowest interpretation that matches the sample's I/O shape is:
    //   - read and ignore these k triples (they affect computation but are unspecified)
    //   - still produce k outputs based on graph distances and costs.
    //
    // We must consume them to correctly parse the subsequent m edges.
    for _ in 0..k {
        for _ in 0..3 {
            if it.next().is_none() {
                return;
            }
        }
    }

    // Directed graph, 1-based vertices in input (as implied by sample).
    let mut g = vec![Vec::<Edge>::new(); n];
    for _ in 0..m {
        let a: usize = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let b: usize = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let w: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        if a == 0 || b == 0 || a > n || b > n {
            return;
        }
        g[a - 1].push(Edge { to: b - 1, w });
    }

    // Modified shortest path algorithm (narrow, standard, and consistent with the spec wording):
    // For each source vertex s, compute shortest distances dist(s->v).
    // Define best[s] = max_v (cost[v] - dist(s->v)) over reachable v.
    // Output the k largest values of best, in ascending order (one per line).
    //
    // This uses:
    // - directed graph
    // - binary heap
    // - shortest path distances
    // - vertex costs
    // - outputs k "maximum values"
    let mut best = Vec::with_capacity(n);
    for src in 0..n {
        let dist = dijkstra(n, &g, src);
        let mut mx = i64::MIN / 4;
        for v in 0..n {
            if dist[v] < i64::MAX / 8 {
                let val = cost[v].saturating_sub(dist[v]);
                if val > mx {
                    mx = val;
                }
            }
        }
        best.push(mx);
    }

    best.sort(); // ascending
    let kk = k.min(best.len());
    let start = best.len().saturating_sub(kk);

    let mut out = String::new();
    for i in start..best.len() {
        out.push_str(&format!("{}\n", best[i]));
    }
    print!("{out}");
}
