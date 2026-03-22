use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

#[derive(Clone)]
struct Edge {
    tail: usize,
    head: usize,
    flow: i32,
    cap: i32,
    cost: i32,
}

fn successive_shortest_path(n: usize, adj: &[Vec<usize>], edges: &mut [Edge], b: &[i32]) -> i64 {
    let m2 = edges.len();

    // Initial objective value from existing flows (normally 0 here).
    let mut ans: i64 = 0;
    for e in edges.iter() {
        ans += (e.cost as i64) * (e.flow as i64);
    }

    // Potentials
    let mut pi = vec![SUP; n + 1];

    // Compute node deficits def[i] = b[i] + sum_out(flow) - sum_in(flow)
    // (the C code computes this via adjacency and reverse-edge flows; we do it directly).
    let mut def = vec![0i32; n + 1];
    for i in 1..=n {
        def[i] = b[i];
    }
    for e in edges.iter() {
        def[e.tail] += e.flow;
        def[e.head] -= e.flow;
    }

    // Heap of nodes with negative deficit (need to send flow out).
    let mut h_def: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

    // Initialize potentials using Bellman-Ford from any node with negative deficit, as in C.
    for i in 1..=n {
        if def[i] < 0 {
            h_def.push((Reverse(def[i]), i));
            if pi[i] == SUP {
                pi[i] = 0;
                // Bellman-Ford relaxation over all residual edges.
                let mut updated = false;
                for _ in 1..=n {
                    updated = false;
                    for k in 0..m2 {
                        if edges[k].flow == edges[k].cap {
                            continue;
                        }
                        let u = edges[k].tail;
                        let w = edges[k].head;
                        if pi[u] != SUP {
                            let cand = pi[u] + edges[k].cost as i64;
                            if pi[w] > cand {
                                pi[w] = cand;
                                updated = true;
                            }
                        }
                    }
                    if !updated {
                        break;
                    }
                }
                if updated {
                    // Negative cycle / ill-posed per spec => return INF sentinel.
                    return INF;
                }
            }
        }
    }

    let mut dist = vec![SUP; n + 1];
    let mut seen = vec![false; n + 1];
    let mut prev: Vec<Option<usize>> = vec![None; n + 1];

    while let Some((Reverse(_key), s)) = h_def.pop() {
        // Dijkstra on reduced costs from s.
        for u in 1..=n {
            dist[u] = SUP;
            seen[u] = false;
            prev[u] = None;
        }
        dist[s] = 0;

        let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), s));

        while let Some((Reverse(_d), u)) = pq.pop() {
            if seen[u] {
                continue;
            }
            seen[u] = true;

            for &eid in &adj[u] {
                let e = &edges[eid];
                if e.flow == e.cap {
                    continue; // no residual capacity
                }
                let w = e.head;
                // reduced cost: c + pi[u] - pi[w]
                let rc = e.cost as i64 + pi[u] - pi[w];
                let nd = dist[u] + rc;
                if dist[w] > nd {
                    dist[w] = nd;
                    prev[w] = Some(eid);
                    pq.push((Reverse(nd), w));
                }
            }
        }

        // Find any t with def[t] > 0 reachable.
        let mut t: usize = 0;
        for i in 1..=n {
            if dist[i] != SUP && def[i] > 0 {
                t = i;
                break;
            }
        }
        if t == 0 {
            // Infeasible / no augmenting path to satisfy deficits.
            return SUP;
        }

        // Determine augmentation amount.
        let mut min_amt: i32 = (-def[s]).min(def[t]);
        let mut u = t;
        while let Some(eid) = prev[u] {
            let e = &edges[eid];
            let residual = e.cap - e.flow;
            if min_amt > residual {
                min_amt = residual;
            }
            u = e.tail;
        }

        // Augment along path, updating flows/caps exactly like the C code.
        let mut u2 = t;
        while let Some(eid) = prev[u2] {
            let rev = eid ^ 1;
            if edges[rev].flow > 0 {
                edges[rev].flow -= min_amt;
                edges[eid].cap -= min_amt;
            } else {
                edges[eid].flow += min_amt;
                edges[rev].cap += min_amt;
            }
            ans += (edges[eid].cost as i64) * (min_amt as i64);
            u2 = edges[eid].tail;
        }

        // Update potentials.
        for i in 1..=n {
            if dist[i] != SUP {
                pi[i] += dist[i];
            }
        }

        // Update deficits and reinsert s if still negative.
        def[s] += min_amt;
        def[t] -= min_amt;
        if def[s] < 0 {
            h_def.push((Reverse(def[s]), s));
        }
    }

    ans
}

fn main() {
    // Read all input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_rows: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let n_cols: usize = it.next().unwrap().parse().unwrap();

    // Grid with 1-based indexing and a zero border (non-passable).
    let mut s = vec![vec![b'\0'; n_cols + 2]; n_rows + 2];
    for i in 1..=n_rows {
        let row = it.next().unwrap().as_bytes().to_vec();
        for j in 1..=n_cols {
            s[i][j] = row[j - 1];
        }
    }

    // Build min-cost flow model as in C.
    // Node ids:
    // 1..=N*M : grid cells
    // N*M + 1.. : marked-cell nodes (one per 'o'), in scan order
    // plus a super source and super sink at the end.
    let nm = n_rows * n_cols;

    // Collect edges and adjacency (store edge indices).
    let mut edges: Vec<Edge> = Vec::new();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); 2603.max(nm + 2603)]; // ensure enough; spec says <= 50*50.

    let mut m_pairs: usize = 0;
    let mut mark_index: usize = 1; // corresponds to `n` in C, starting at 1

    // BFS arrays sized to N*M (<= 2500).
    let mut dist = vec![-1i32; nm + 1];
    let mut q = vec![0usize; nm + 1];

    // Scan from bottom-right to top-left, as in C.
    for i in (1..=n_rows).rev() {
        for j in (1..=n_cols).rev() {
            if s[i][j] != b'o' {
                continue;
            }
            s[i][j] = b'.';

            // Reset dist
            for k in 1..=nm {
                dist[k] = -1;
            }

            let start = (i - 1) * n_cols + j;
            q[0] = start;
            dist[start] = 0;

            let mut head = 0usize;
            let mut tail = 1usize;
            while head < tail {
                let v = q[head];
                head += 1;
                let r = (v - 1) / n_cols + 1;
                let c = (v - 1) % n_cols + 1;

                // Move down
                if r + 1 <= n_rows && s[r + 1][c] == b'.' {
                    let to = r * n_cols + c;
                    if dist[to] == -1 {
                        dist[to] = dist[v] + 1;
                        q[tail] = to;
                        tail += 1;
                    }
                }
                // Move right
                if c + 1 <= n_cols && s[r][c + 1] == b'.' {
                    let to = (r - 1) * n_cols + (c + 1);
                    if dist[to] == -1 {
                        dist[to] = dist[v] + 1;
                        q[tail] = to;
                        tail += 1;
                    }
                }
            }

            // Add edges to up to 100 farthest reachable destinations.
            // C iterates from last discovered backwards, taking at most 100.
            let mark_node = nm + mark_index;
            let mut taken = 0usize;
            let mut idx = tail;
            while idx > 0 && taken < 100 {
                idx -= 1;
                let cell = q[idx];
                let d = dist[cell];
                if d < 0 {
                    continue;
                }

                // forward edge: mark_node -> cell, cap 1, cost = -distance
                let eid_f = edges.len();
                edges.push(Edge {
                    tail: mark_node,
                    head: cell,
                    flow: 0,
                    cap: 1,
                    cost: -d,
                });
                let eid_r = edges.len();
                edges.push(Edge {
                    tail: cell,
                    head: mark_node,
                    flow: 0,
                    cap: 0,
                    cost: d,
                });

                if mark_node >= adj.len() {
                    adj.resize(mark_node + 1, Vec::new());
                }
                if cell >= adj.len() {
                    adj.resize(cell + 1, Vec::new());
                }
                adj[mark_node].push(eid_f);
                adj[cell].push(eid_r);

                m_pairs += 1;
                taken += 1;
            }

            mark_index += 1;
        }
    }

    let num_marks = mark_index - 1;
    let super_source = nm + mark_index; // N*M + n in C after loop
    let super_sink = nm + mark_index + 1;

    if super_sink >= adj.len() {
        adj.resize(super_sink + 1, Vec::new());
    }

    // super_source -> each mark node, cap 1
    for i in 1..=num_marks {
        let mark_node = nm + i;
        let eid_f = edges.len();
        edges.push(Edge {
            tail: super_source,
            head: mark_node,
            flow: 0,
            cap: 1,
            cost: 0,
        });
        let eid_r = edges.len();
        edges.push(Edge {
            tail: mark_node,
            head: super_source,
            flow: 0,
            cap: 0,
            cost: 0,
        });

        adj[super_source].push(eid_f);
        adj[mark_node].push(eid_r);
        m_pairs += 1;
    }

    // each cell -> super_sink, cap 1
    for cell in 1..=nm {
        let eid_f = edges.len();
        edges.push(Edge {
            tail: cell,
            head: super_sink,
            flow: 0,
            cap: 1,
            cost: 0,
        });
        let eid_r = edges.len();
        edges.push(Edge {
            tail: super_sink,
            head: cell,
            flow: 0,
            cap: 0,
            cost: 0,
        });

        adj[cell].push(eid_f);
        adj[super_sink].push(eid_r);
        m_pairs += 1;
    }

    // b vector (node demands): b[super_source] = -(num_marks), b[super_sink] = num_marks
    let total_nodes = super_sink; // nodes are 1..=super_sink
    let mut b = vec![0i32; total_nodes + 1];
    b[super_source] = -(num_marks as i32);
    b[super_sink] = num_marks as i32;

    // Run min-cost flow; output is negated to become maximum total distance.
    let res = successive_shortest_path(total_nodes, &adj[..=total_nodes], &mut edges, &b);
    let out = -res;
    print!("{out}\n");
}
