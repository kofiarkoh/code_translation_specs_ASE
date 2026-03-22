use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

#[derive(Clone, Debug)]
struct Edge {
    tail: usize,
    head: usize,
    cap: i32,
    flow: i32,
    cost: i64,
}

/// Successive shortest path for min-cost circulation with node demands `b`.
/// Returns:
/// - `INF` if negative cycle detected during initial potential computation (ill-posed)
/// - `SUP` if cannot find an augmenting path from a deficit node to any surplus node (infeasible)
/// - otherwise the minimum cost (can be negative)
///
/// This is a direct semantic translation of the C routine, including its
/// somewhat unusual residual/flow update rules.
fn successive_shortest_path(n: usize, m: usize, adj: &[Vec<usize>], e: &mut [Edge], b: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    for k in 0..m {
        ans += e[k].cost * (e[k].flow as i64);
    }

    let mut flag = vec![0i32; n + 1];
    let mut def = vec![0i32; n + 1];
    let mut dist = vec![SUP; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev: Vec<Option<usize>> = vec![None; n + 1];

    // Initialize deficits and potentials
    let mut h_def: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new(); // (def, node), min by def
    for i in 1..=n {
        def[i] = b[i];
        for &eid in &adj[i] {
            // def[i] += p->e->flow - e[(p->e->id)^1].flow;
            let rev = eid ^ 1;
            def[i] += e[eid].flow - e[rev].flow;
        }
        if def[i] < 0 {
            h_def.push(Reverse((def[i], i)));

            if pi[i] == SUP {
                pi[i] = 0;
                // Bellman-Ford-like relaxation over all edges up to N times
                let mut j = 1;
                while j <= n {
                    let mut changed = false;
                    for k in 0..m {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost {
                            pi[w] = pi[u] + e[k].cost;
                            changed = true;
                        }
                    }
                    if !changed {
                        break;
                    }
                    j += 1;
                }
                if j > n {
                    return INF;
                }
            }
        }
    }

    while let Some(Reverse((_key, s))) = h_def.pop() {
        for u in 1..=n {
            flag[u] = 0;
            dist[u] = SUP;
            prev[u] = None;
        }
        dist[s] = 0;

        let mut h_dist: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        h_dist.push(Reverse((0, s)));

        // Dijkstra with reduced costs
        while let Some(Reverse((d_u, u))) = h_dist.pop() {
            if flag[u] != 0 {
                continue;
            }
            flag[u] = 1;
            if d_u != dist[u] {
                // stale entry
                continue;
            }

            for &eid in &adj[u] {
                let w = e[eid].head;
                if e[eid].flow == e[eid].cap {
                    continue;
                }
                // dist[w] > dist[u] + cost + pi[u] - pi[w]
                let nd = dist[u]
                    .saturating_add(e[eid].cost)
                    .saturating_add(pi[u])
                    .saturating_sub(pi[w]);
                if dist[w] > nd {
                    dist[w] = nd;
                    prev[w] = Some(eid);
                    h_dist.push(Reverse((nd, w)));
                }
            }
        }

        // Find any reachable node with positive deficit
        let mut t = 1usize;
        while t <= n {
            if dist[t] != SUP && def[t] > 0 {
                break;
            }
            t += 1;
        }
        if t > n {
            return SUP;
        }

        // Determine augmentation amount
        let mut minv: i32 = (-def[s]).min(def[t]);
        let mut u = t;
        while let Some(eid) = prev[u] {
            let residual = e[eid].cap - e[eid].flow;
            if minv > residual {
                minv = residual;
            }
            u = e[eid].tail;
        }

        // Augment along path with the C code's update rules
        let mut u2 = t;
        while let Some(eid) = prev[u2] {
            let rev = eid ^ 1;
            if e[rev].flow > 0 {
                e[rev].flow -= minv;
                e[eid].cap -= minv;
            } else {
                e[eid].flow += minv;
                e[rev].cap += minv;
            }
            ans += e[eid].cost * (minv as i64);
            u2 = e[eid].tail;
        }

        // Update potentials
        for i in 1..=n {
            if dist[i] != SUP {
                pi[i] = pi[i].saturating_add(dist[i]);
            }
        }

        def[s] += minv;
        def[t] -= minv;
        if def[s] < 0 {
            h_def.push(Reverse((def[s], s)));
        }
    }

    ans
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
    let k_limit: i32 = it.next().unwrap().parse().unwrap();

    // 1-based A like C code
    let mut a = vec![vec![0i64; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            a[i][j] = it.next().unwrap().parse::<i64>().unwrap();
        }
    }

    // Build graph as in C:
    // Left nodes: 1..N
    // Right nodes: N+1..2N
    // Source: 2N+1, Sink: 2N+2
    let total_nodes = 2 * n + 2;
    let source = 2 * n + 1;
    let sink = 2 * n + 2;

    // We will store edges in a Vec<Edge>, and adjacency as Vec<Vec<usize>> of edge indices.
    let mut edges: Vec<Edge> = Vec::new();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); total_nodes + 1];

    let mut add_edge_pair = |tail: usize, head: usize, cap_fwd: i32, cost_fwd: i64, cap_rev: i32, cost_rev: i64,
                             edges: &mut Vec<Edge>, adj: &mut Vec<Vec<usize>>| {
        let id = edges.len();
        edges.push(Edge {
            tail,
            head,
            cap: cap_fwd,
            flow: 0,
            cost: cost_fwd,
        });
        edges.push(Edge {
            tail: head,
            head: tail,
            cap: cap_rev,
            flow: 0,
            cost: cost_rev,
        });
        adj[tail].push(id);
        adj[head].push(id ^ 1);
    };

    // Left->Right edges cap=1 cost=-A
    for u in 1..=n {
        for w in (n + 1)..=(2 * n) {
            let col = w - n;
            add_edge_pair(
                u,
                w,
                1,
                -a[u][col],
                0,
                a[u][col],
                &mut edges,
                &mut adj,
            );
        }
    }
    // Source -> Left edges cap=K cost=0
    for w in 1..=n {
        add_edge_pair(source, w, k_limit, 0, 0, 0, &mut edges, &mut adj);
    }
    // Right -> Sink edges cap=K cost=0
    for u in (n + 1)..=(2 * n) {
        add_edge_pair(u, sink, k_limit, 0, 0, 0, &mut edges, &mut adj);
    }

    let m = edges.len();
    let mut b = vec![0i32; total_nodes + 1];

    let mut argmax: i32 = 0;
    let mut best: i64 = 0;

    // Try all i in 1..=N*K (if K==0, loop skipped; best remains 0, argmax 0)
    let max_i = (n as i32).saturating_mul(k_limit).max(0);
    for i in 1..=max_i {
        b[source] = -i;
        b[sink] = i;
        let tmp = -successive_shortest_path(total_nodes, m, &adj, &mut edges, &b);
        if tmp != SUP && best < tmp {
            best = tmp;
            argmax = i;
        }
    }

    // Output best value
    print!("{best}\n");

    // Reconstruct placement for argmax
    let mut grid = vec![vec!['.'; n + 1]; n + 1];
    b[source] = -argmax;
    b[sink] = argmax;

    // Reset flows/caps as in C:
    // for each pair j: e[j*2].flow=0; e[j*2+1].cap=0;
    for j in (0..m).step_by(2) {
        edges[j].flow = 0;
        edges[j ^ 1].cap = 0;
        // Note: we intentionally do not reset other fields; matches C behavior.
    }

    let _ = successive_shortest_path(total_nodes, m, &adj, &mut edges, &b);

    // Mark chosen left->right edges with flow==1
    for j in (0..m).step_by(2) {
        let u = edges[j].tail;
        let w = edges[j].head;
        if u >= 1 && u <= n && w >= n + 1 && w <= 2 * n && edges[j].flow == 1 {
            grid[u][w - n] = 'X';
        }
    }

    for i in 1..=n {
        let line: String = (1..=n).map(|j| grid[i][j]).collect();
        print!("{line}\n");
    }
}
