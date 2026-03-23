use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, Read};

const INF: u64 = u64::MAX / 4;

#[derive(Clone)]
struct Edge {
    to: usize,
    w: u64,
}

fn dijkstra(n: usize, g: &[Vec<Edge>], src: usize) -> Vec<u64> {
    let mut dist = vec![INF; n];
    let mut pq: BinaryHeap<(Reverse<u64>, usize)> = BinaryHeap::new();
    dist[src] = 0;
    pq.push((Reverse(0), src));

    while let Some((Reverse(d), u)) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        for e in &g[u] {
            let nd = d.saturating_add(e.w);
            if nd < dist[e.to] {
                dist[e.to] = nd;
                pq.push((Reverse(nd), e.to));
            }
        }
    }
    dist
}

fn main() {
    // Input format is treated as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let s: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let r: usize = it.next().unwrap().parse().unwrap();

    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); s];
    for _ in 0..r {
        let u1: usize = it.next().unwrap().parse().unwrap();
        let v1: usize = it.next().unwrap().parse().unwrap();
        let w: u64 = it.next().unwrap().parse().unwrap();
        let u = u1 - 1;
        let v = v1 - 1;
        g[u].push(Edge { to: v, w });
        g[v].push(Edge { to: u, w });
    }

    let a1: usize = it.next().unwrap().parse().unwrap();
    let b1: usize = it.next().unwrap().parse().unwrap();
    let a = a1 - 1;
    let b = b1 - 1;

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut qs: Vec<(usize, usize)> = Vec::with_capacity(q);
    for _ in 0..q {
        let x1: usize = it.next().unwrap().parse().unwrap();
        let y1: usize = it.next().unwrap().parse().unwrap();
        qs.push((x1 - 1, y1 - 1));
    }

    // 1) dist[] from a
    let dist = dijkstra(s, &g, a);

    // 2) Build relevant shortest-path-consistent directed graph by reverse exploration from b.
    // We mark nodes that can lie on at least one shortest a->b path.
    let mut relevant = vec![false; s];
    let mut dir: Vec<Vec<usize>> = vec![Vec::new(); s]; // directed edges u -> v
    let mut indeg: Vec<usize> = vec![0; s];

    let mut dq: VecDeque<usize> = VecDeque::new();
    relevant[b] = true;
    dq.push_back(b);

    while let Some(cur) = dq.pop_front() {
        // Explore undirected neighbors; if neighbor can precede cur on a shortest path, add edge neighbor -> cur.
        for e in &g[cur] {
            let prev = e.to;
            // Condition: dist[cur] == dist[prev] + w(prev,cur)
            if dist[prev] != INF && dist[cur] == dist[prev].saturating_add(e.w) {
                // Record directed edge prev -> cur (forward in dist).
                dir[prev].push(cur);
                indeg[cur] += 1;

                if !relevant[prev] {
                    relevant[prev] = true;
                    dq.push_back(prev);
                }
            }
        }
    }

    // 3) Topological order via Kahn-style queue, restricted to relevant nodes, starting from a.
    // Only nodes reachable from a within the relevant directed graph are included.
    let mut indeg_work = indeg.clone();
    let mut topo: Vec<usize> = Vec::new();
    let mut qk: VecDeque<usize> = VecDeque::new();

    if relevant[a] && indeg_work[a] == 0 {
        qk.push_back(a);
    }

    let mut in_topo = vec![false; s];
    while let Some(u) = qk.pop_front() {
        if in_topo[u] {
            continue;
        }
        in_topo[u] = true;
        topo.push(u);
        for &v in &dir[u] {
            if !relevant[v] {
                continue;
            }
            if indeg_work[v] > 0 {
                indeg_work[v] -= 1;
                if indeg_work[v] == 0 {
                    qk.push_back(v);
                }
            }
        }
    }

    // 4) Answer queries in batches of up to 64 using bitsets.
    let mut out = String::new();
    let mut node_bits: Vec<u64> = vec![0; s];

    let mut i = 0usize;
    while i < q {
        let batch = (q - i).min(64);

        // Reset per-node bitsets for this batch.
        // Narrowest faithful approach: clear all nodes (O(S) per batch), as spec does not define optimizations.
        for v in &mut node_bits {
            *v = 0;
        }

        // Initialize start bits at x nodes.
        for j in 0..batch {
            let (x, _) = qs[i + j];
            node_bits[x] |= 1u64 << j;
        }

        // Propagate along topo order.
        for &u in &topo {
            let bits_u = node_bits[u];
            if bits_u == 0 {
                continue;
            }
            for &v in &dir[u] {
                if !relevant[v] {
                    continue;
                }
                node_bits[v] |= bits_u;
            }
        }

        // Emit answers for this batch.
        for j in 0..batch {
            let (_, y) = qs[i + j];
            let yes = (node_bits[y] >> j) & 1 == 1;
            if yes {
                out.push_str("Yes\n");
            } else {
                out.push_str("No\n");
            }
        }

        i += batch;
    }

    print!("{out}");
}
