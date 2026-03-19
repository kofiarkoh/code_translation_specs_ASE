use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Data {
    key: i64,
    id: i32,
}

// 1-indexed min-heap by key (matches C heap behavior)
struct MinHeap {
    obj: Vec<Data>,
    size: usize,
}
impl MinHeap {
    fn with_capacity(n: usize) -> Self {
        // index 0 unused
        Self {
            obj: vec![Data::default(); n + 1],
            size: 0,
        }
    }
    fn push(&mut self, x: Data) {
        self.size += 1;
        let mut i = self.size;
        let mut j = i >> 1;
        self.obj[i] = x;
        while j > 0 {
            if self.obj[i].key < self.obj[j].key {
                self.obj.swap(i, j);
                i = j;
                j >>= 1;
            } else {
                break;
            }
        }
    }
    fn pop(&mut self) -> Data {
        let mut i = 1usize;
        let mut j = 2usize;
        let output = self.obj[1];
        self.obj[1] = self.obj[self.size];
        self.size -= 1;
        while j <= self.size {
            if j < self.size && self.obj[j ^ 1].key < self.obj[j].key {
                j ^= 1;
            }
            if self.obj[j].key < self.obj[i].key {
                self.obj.swap(j, i);
                i = j;
                j <<= 1;
            } else {
                break;
            }
        }
        output
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[derive(Clone, Copy, Default)]
struct Edge {
    id: i32,
    head: i32,
    tail: i32,
    flow: i32,
    cap: i32,
    cost: i32,
}

#[derive(Clone, Copy, Default)]
struct ListNode {
    next: i32, // index into nodes, -1 for null
    e: i32,    // index into edges
}

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(
    n: usize,
    m: usize,
    adj: &[i32],          // head indices into nodes, -1 for null; 1..=n used
    nodes: &[ListNode],   // adjacency nodes
    edges: &mut [Edge],   // length m
    b: &[i32],            // 1..=n used
) -> i64 {
    let mut flag = vec![0i32; n + 1];
    let mut def = vec![0i32; n + 1];
    let mut dist = vec![SUP; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev = vec![-1i32; n + 1]; // store edge index, -1 for NULL

    let mut ans: i64 = 0;
    for i in 0..m {
        ans += (edges[i].cost as i64) * (edges[i].flow as i64);
    }

    for i in 1..=n {
        pi[i] = SUP;
    }

    let mut h_def = MinHeap::with_capacity(n + 2);
    let mut h_dist = MinHeap::with_capacity(m * 2 + 2);

    // Initial deficits and potentials (Bellman-Ford-like)
    for i in 1..=n {
        def[i] = b[i];
        let mut p = adj[i];
        while p != -1 {
            let node = nodes[p as usize];
            let ei = node.e as usize;
            let rev = (edges[ei].id as usize) ^ 1;
            def[i] += edges[ei].flow - edges[rev].flow;
            p = node.next;
        }

        if def[i] < 0 {
            h_def.push(Data {
                key: def[i] as i64,
                id: i as i32,
            });

            if pi[i] == SUP {
                pi[i] = 0;
                let mut j_done = 0usize;
                for j in 1..=n {
                    j_done = j;
                    let mut changed = 0i32;
                    for k in 0..m {
                        if edges[k].flow == edges[k].cap {
                            continue;
                        }
                        let u = edges[k].tail as usize;
                        let w = edges[k].head as usize;
                        if pi[u] != SUP {
                            let cand = pi[u] + edges[k].cost as i64;
                            if pi[w] > cand {
                                changed = 1;
                                pi[w] = cand;
                            }
                        }
                    }
                    if changed == 0 {
                        break;
                    }
                }
                if j_done > n {
                    return INF;
                }
            }
        }
    }

    while !h_def.is_empty() {
        let d = h_def.pop();
        let s = d.id as usize;

        for u in 1..=n {
            flag[u] = 0;
            dist[u] = SUP;
            prev[u] = -1;
        }
        dist[s] = 0;
        h_dist.size = 0;
        h_dist.push(Data { key: 0, id: s as i32 });

        // Dijkstra with reduced costs
        while !h_dist.is_empty() {
            let d2 = h_dist.pop();
            let u = d2.id as usize;
            if flag[u] != 0 {
                continue;
            }
            flag[u] = 1;

            let mut p = adj[u];
            while p != -1 {
                let node = nodes[p as usize];
                let ei = node.e as usize;
                let w = edges[ei].head as usize;

                if edges[ei].flow == edges[ei].cap {
                    p = node.next;
                    continue;
                }

                // dist[w] > dist[u] + cost + pi[u] - pi[w]
                let cand = dist[u]
                    .wrapping_add(edges[ei].cost as i64)
                    .wrapping_add(pi[u])
                    .wrapping_sub(pi[w]);
                if dist[w] > cand {
                    dist[w] = cand;
                    prev[w] = ei as i32;
                    h_dist.push(Data {
                        key: dist[w],
                        id: w as i32,
                    });
                }

                p = node.next;
            }
        }

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

        let mut minv: i32 = if -def[s] < def[t] { -def[s] } else { def[t] };
        let mut u = t;
        while prev[u] != -1 {
            let ei = prev[u] as usize;
            let residual = edges[ei].cap - edges[ei].flow;
            if minv > residual {
                minv = residual;
            }
            u = edges[ei].tail as usize;
        }

        u = t;
        while prev[u] != -1 {
            let ei = prev[u] as usize;
            let rev = (ei ^ 1) as usize;
            if edges[rev].flow > 0 {
                edges[rev].flow -= minv;
                edges[ei].cap -= minv;
            } else {
                edges[ei].flow += minv;
                edges[rev].cap += minv;
            }
            ans += (edges[ei].cost as i64) * (minv as i64);
            u = edges[ei].tail as usize;
        }

        for i in 1..=n {
            if dist[i] != SUP {
                pi[i] = pi[i].wrapping_add(dist[i]);
            }
        }

        def[s] += minv;
        def[t] -= minv;
        if def[s] < 0 {
            h_def.push(Data {
                key: def[s] as i64,
                id: s as i32,
            });
        }
    }

    ans
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_in: usize = it.next().unwrap().parse().unwrap();
    let k_in: i32 = it.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0i32; 51]; 51];
    for i in 1..=n_in {
        for j in 1..=n_in {
            a[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // Build graph
    // Node indices: 1..=2N+2
    let n_nodes = n_in * 2 + 2;
    let source = n_in * 2 + 1;
    let sink = n_in * 2 + 2;

    // Maximum sizes as in C: edges/nodes arrays sized 6000
    let mut edges: Vec<Edge> = vec![Edge::default(); 6000];
    let mut nodes: Vec<ListNode> = vec![ListNode::default(); 6000];
    let mut adj: Vec<i32> = vec![-1; 103];

    let mut k: usize = 0;

    // bipartite edges u -> w (N+1..2N)
    for u in 1..=n_in {
        for w in (n_in + 1)..=(n_in * 2) {
            let cost = -a[u][w - n_in];
            edges[k * 2] = Edge {
                id: (k * 2) as i32,
                tail: u as i32,
                head: w as i32,
                flow: 0,
                cap: 1,
                cost,
            };
            edges[k * 2 + 1] = Edge {
                id: (k * 2 + 1) as i32,
                tail: w as i32,
                head: u as i32,
                flow: 0,
                cap: 0,
                cost: -cost,
            };

            nodes[k * 2] = ListNode {
                e: (k * 2) as i32,
                next: adj[u],
            };
            nodes[k * 2 + 1] = ListNode {
                e: (k * 2 + 1) as i32,
                next: adj[w],
            };
            adj[u] = (k * 2) as i32;
            adj[w] = (k * 2 + 1) as i32;

            k += 1;
        }
    }

    // source -> left
    for w in 1..=n_in {
        edges[k * 2] = Edge {
            id: (k * 2) as i32,
            tail: source as i32,
            head: w as i32,
            flow: 0,
            cap: k_in,
            cost: 0,
        };
        edges[k * 2 + 1] = Edge {
            id: (k * 2 + 1) as i32,
            tail: w as i32,
            head: source as i32,
            flow: 0,
            cap: 0,
            cost: 0,
        };

        nodes[k * 2] = ListNode {
            e: (k * 2) as i32,
            next: adj[source],
        };
        nodes[k * 2 + 1] = ListNode {
            e: (k * 2 + 1) as i32,
            next: adj[w],
        };
        adj[source] = (k * 2) as i32;
        adj[w] = (k * 2 + 1) as i32;

        k += 1;
    }

    // right -> sink
    for u in (n_in + 1)..=(n_in * 2) {
        edges[k * 2] = Edge {
            id: (k * 2) as i32,
            tail: u as i32,
            head: sink as i32,
            flow: 0,
            cap: k_in,
            cost: 0,
        };
        edges[k * 2 + 1] = Edge {
            id: (k * 2 + 1) as i32,
            tail: sink as i32,
            head: u as i32,
            flow: 0,
            cap: 0,
            cost: 0,
        };

        nodes[k * 2] = ListNode {
            e: (k * 2) as i32,
            next: adj[u],
        };
        nodes[k * 2 + 1] = ListNode {
            e: (k * 2 + 1) as i32,
            next: adj[sink],
        };
        adj[u] = (k * 2) as i32;
        adj[sink] = (k * 2 + 1) as i32;

        k += 1;
    }

    let m_edges = k * 2;

    let mut b = vec![0i32; 103];
    let mut argmax: i32 = 0;
    let mut maxv: i64 = 0;

    for i in 1..=(n_in as i32 * k_in) {
        b[source] = -i;
        b[sink] = i;
        let tmp = -succesive_shortest_path(n_nodes, m_edges, &adj, &nodes, &mut edges, &b);
        if tmp != SUP && maxv < tmp {
            maxv = tmp;
            argmax = i;
        }
    }

    let mut out = String::new();
    out.push_str(&format!("{maxv}\n"));

    // Build answer grid
    let mut ans = vec![vec!['.'; 52]; 52];
    for i in 1..=n_in {
        for j in 1..=n_in {
            ans[i][j] = '.';
        }
    }

    b[source] = -argmax;
    b[sink] = argmax;

    for j in 0..k {
        edges[j * 2].flow = 0;
        edges[j * 2 + 1].cap = 0;
    }

    succesive_shortest_path(n_nodes, m_edges, &adj, &nodes, &mut edges, &b);

    for j in 0..k {
        let u = edges[j * 2].tail as usize;
        let w = edges[j * 2].head as usize;
        if u <= n_in && w <= n_in * 2 && edges[j * 2].flow == 1 {
            ans[u][w - n_in] = 'X';
        }
    }

    for i in 1..=n_in {
        let line: String = (1..=n_in).map(|j| ans[i][j]).collect();
        out.push_str(&line);
        out.push('\n');
    }

    print!("{out}");
    // stdout flush is implicit at program end; C calls fflush(stdout) but output is identical.
}
