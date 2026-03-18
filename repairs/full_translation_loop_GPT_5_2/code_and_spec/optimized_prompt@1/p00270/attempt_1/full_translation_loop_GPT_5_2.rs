use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: i32 = 0x1010_1010;

fn main() {
    // Fast input: read all bytes
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let s: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let r: usize = it.next().unwrap().parse().unwrap();

    // Build undirected weighted graph
    let mut g: Vec<Vec<(usize, i32)>> = vec![Vec::new(); s];
    for _ in 0..r {
        let u: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let w: i32 = it.next().unwrap().parse().unwrap();
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
    let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
    let qn: usize = it.next().unwrap().parse().unwrap();

    let mut c: Vec<usize> = vec![0; qn];
    let mut d: Vec<usize> = vec![0; qn];
    for i in 0..qn {
        c[i] = it.next().unwrap().parse::<usize>().unwrap() - 1;
        d[i] = it.next().unwrap().parse::<usize>().unwrap() - 1;
    }

    // Dijkstra from a
    let dist = dijkstra(&g, a);

    // Build shortest-path DAG edges (reversed orientation as in C: e -> s when e is predecessor of s)
    // and compute a topological order starting from `a` within the reachable subgraph from `b` backwards.
    let (to2, ord) = tsort_like_c(&g, &dist, a, b);

    // Batch queries in groups of 64 using bitset DP along topological order.
    let mut out = String::new();
    let blocks = (qn + 63) >> 6;
    for bi in 0..blocks {
        let mut dp: Vec<u64> = vec![0u64; s];

        for j in 0..64 {
            let idx = (bi << 6) + j;
            if idx < qn {
                dp[c[idx]] |= 1u64 << j;
            }
        }

        for &u in &ord {
            let bits = dp[u];
            if bits == 0 {
                continue;
            }
            for &v in &to2[u] {
                dp[v] |= bits;
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

fn dijkstra(g: &[Vec<(usize, i32)>], start: usize) -> Vec<i32> {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

    dist[start] = 0;
    pq.push((Reverse(0), start));

    while let Some((Reverse(d), u)) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        for &(v, w) in &g[u] {
            let nd = d.saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                pq.push((Reverse(nd), v));
            }
        }
    }
    dist
}

fn tsort_like_c(
    g: &[Vec<(usize, i32)>],
    dist: &[i32],
    start: usize,
    goal: usize,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    let n = g.len();

    // Phase 1: BFS/queue from goal following predecessor edges on shortest paths,
    // building to2[e].push(s) where e is predecessor of s.
    let mut to2: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut mk = vec![false; n];
    let mut q: Vec<usize> = Vec::with_capacity(n);
    q.push(goal);
    let mut top = 0usize;
    while top < q.len() {
        let s = q[top];
        top += 1;
        if mk[s] {
            continue;
        }
        for &(e, w) in &g[s] {
            // If e is a predecessor of s on some shortest path from start:
            if dist[s] == dist[e].saturating_add(w) {
                q.push(e);
                to2[e].push(s);
            }
        }
        mk[s] = true;
    }

    // Phase 2: Kahn-like topological traversal starting from `start` over to2 edges.
    let mut deg = vec![0i32; n];
    for u in 0..n {
        for &v in &to2[u] {
            deg[v] += 1;
        }
    }

    let mut ord: Vec<usize> = Vec::new();
    let mut q2: Vec<usize> = Vec::with_capacity(n);
    q2.push(start);
    let mut top2 = 0usize;
    while top2 < q2.len() {
        let u = q2[top2];
        top2 += 1;
        ord.push(u);
        for &v in &to2[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                q2.push(v);
            }
        }
    }

    (to2, ord)
}
