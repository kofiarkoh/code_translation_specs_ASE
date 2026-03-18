use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Data {
    key: i64,
    id: usize,
}

struct MinHeap {
    obj: Vec<Data>, // 1-based indexing; obj[0] unused
    size: usize,
}

impl MinHeap {
    fn new(capacity: usize) -> Self {
        let mut obj = Vec::with_capacity(capacity + 1);
        obj.resize(capacity + 1, Data::default());
        Self { obj, size: 0 }
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
                self.obj.swap(i, j);
                i = j;
                j <<= 1;
            } else {
                break;
            }
        }
        output
    }
}

#[derive(Clone, Copy, Default)]
struct Edge {
    id: usize,
    head: usize,
    tail: usize,
    flow: i32,
    cap: i32,
    cost: i32,
}

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(
    n: usize,
    m: usize, // number of directed edges in `e` (C passes k*2)
    adj: &Vec<Vec<usize>>, // adjacency list of edge indices
    e: &mut [Edge],
    b: &[i32],
) -> i64 {
    let mut flag = vec![0i32; n + 1];
    let mut def = vec![0i32; n + 1];
    let mut dist = vec![SUP; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev: Vec<Option<usize>> = vec![None; n + 1];

    let mut ans: i64 = 0;
    for i in 0..m {
        ans += (e[i].cost as i64) * (e[i].flow as i64);
    }

    for i in 1..=n {
        pi[i] = SUP;
    }

    let mut h_def = MinHeap::new(n + 2);
    let mut h_dist = MinHeap::new(m * 2 + 2);

    for i in 1..=n {
        def[i] = b[i];
        for &eid in &adj[i] {
            // def[i] += p->e->flow - e[(p->e->id)^1].flow;
            let f1 = e[eid].flow;
            let f2 = e[eid ^ 1].flow;
            def[i] = def[i].wrapping_add(f1.wrapping_sub(f2));
        }

        if def[i] < 0 {
            h_def.push(Data {
                key: def[i] as i64,
                id: i,
            });

            if pi[i] == SUP {
                pi[i] = 0;
                let mut j_done = 0usize;
                for j in 1..=n {
                    flag[0] = 0;
                    for k in 0..m {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP {
                            let cand = pi[u] + (e[k].cost as i64);
                            if pi[w] > cand {
                                flag[0] = 1;
                                pi[w] = cand;
                            }
                        }
                    }
                    j_done = j;
                    if flag[0] == 0 {
                        break;
                    }
                }
                if j_done > n {
                    return INF;
                }
            }
        }
    }

    while h_def.size > 0 {
        let d = h_def.pop();
        let s = d.id;

        for u in 1..=n {
            flag[u] = 0;
            dist[u] = SUP;
            prev[u] = None;
        }
        dist[s] = 0;
        h_dist.size = 0;
        h_dist.push(Data { key: 0, id: s });

        while h_dist.size > 0 {
            let d2 = h_dist.pop();
            let u = d2.id;
            if flag[u] != 0 {
                continue;
            }
            flag[u] = 1;

            for &eid in &adj[u] {
                let w = e[eid].head;
                if e[eid].flow == e[eid].cap {
                    continue;
                }
                let cand = dist[u] + (e[eid].cost as i64) + pi[u] - pi[w];
                if dist[w] > cand {
                    dist[w] = cand;
                    prev[w] = Some(eid);
                    h_dist.push(Data { key: dist[w], id: w });
                }
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
        while let Some(eid) = prev[u] {
            let residual = e[eid].cap - e[eid].flow;
            if minv > residual {
                minv = residual;
            }
            u = e[eid].tail;
        }

        u = t;
        while let Some(eid) = prev[u] {
            let i = e[eid].id;
            if e[i ^ 1].flow > 0 {
                e[i ^ 1].flow -= minv;
                e[i].cap -= minv;
            } else {
                e[i].flow += minv;
                e[i ^ 1].cap += minv;
            }
            ans += (e[i].cost as i64) * (minv as i64);
            u = e[eid].tail;
        }

        for i in 1..=n {
            if dist[i] != SUP {
                pi[i] += dist[i];
            }
        }

        def[s] += minv;
        def[t] -= minv;
        if def[s] < 0 {
            h_def.push(Data {
                key: def[s] as i64,
                id: s,
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

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k_in: i32 = it.next().unwrap().parse().unwrap();

    // A[51][51] with 1-based indexing
    let mut a = vec![vec![0i32; 51]; 51];
    for i in 1..=n {
        for j in 1..=n {
            a[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // Build graph
    let node_count = n * 2 + 2; // nodes 1..=2N+2
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); node_count + 1];

    // Maximum edges: for N<=50, k_total <= 2500 + 50 + 50 = 2600, directed edges = 5200
    let mut edges: Vec<Edge> = Vec::with_capacity(6000);

    let mut k_total: usize = 0;

    // u in [1..N], w in [N+1..2N]
    for u in 1..=n {
        for w in (n + 1)..=(n * 2) {
            let cost = -a[u][w - n];
            let id0 = k_total * 2;
            let id1 = k_total * 2 + 1;

            edges.push(Edge {
                id: id0,
                tail: u,
                head: w,
                cap: 1,
                cost,
                flow: 0,
            });
            edges.push(Edge {
                id: id1,
                tail: w,
                head: u,
                cap: 0,
                cost: -cost,
                flow: 0,
            });

            adj[u].push(id0);
            adj[w].push(id1);

            k_total += 1;
        }
    }

    // source = 2N+1 to left nodes 1..N with cap K
    let source = n * 2 + 1;
    for w in 1..=n {
        let id0 = k_total * 2;
        let id1 = k_total * 2 + 1;

        edges.push(Edge {
            id: id0,
            tail: source,
            head: w,
            cap: k_in,
            cost: 0,
            flow: 0,
        });
        edges.push(Edge {
            id: id1,
            tail: w,
            head: source,
            cap: 0,
            cost: 0,
            flow: 0,
        });

        adj[source].push(id0);
        adj[w].push(id1);

        k_total += 1;
    }

    // right nodes N+1..2N to sink = 2N+2 with cap K
    let sink = n * 2 + 2;
    for u in (n + 1)..=(n * 2) {
        let id0 = k_total * 2;
        let id1 = k_total * 2 + 1;

        edges.push(Edge {
            id: id0,
            tail: u,
            head: sink,
            cap: k_in,
            cost: 0,
            flow: 0,
        });
        edges.push(Edge {
            id: id1,
            tail: sink,
            head: u,
            cap: 0,
            cost: 0,
            flow: 0,
        });

        adj[u].push(id0);
        adj[sink].push(id1);

        k_total += 1;
    }

    let mut b = vec![0i32; node_count + 1];
    let mut argmax: i32 = 0;
    let mut maxv: i64 = 0;

    for i in 1..=(n as i32 * k_in) {
        b[source] = -i;
        b[sink] = i;
        let tmp = -succesive_shortest_path(node_count, k_total * 2, &adj, &mut edges, &b);
        if tmp != SUP && maxv < tmp {
            maxv = tmp;
            argmax = i;
        }
    }

    print!("{maxv}\n");

    // Build answer grid
    let mut ans = vec![vec!['.'; n + 1]; n + 1]; // 1-based columns
    b[source] = -argmax;
    b[sink] = argmax;

    // Reset flows/caps as in C
    for j in 0..k_total {
        edges[j * 2].flow = 0;
        edges[j * 2 + 1].cap = 0;
    }

    succesive_shortest_path(node_count, k_total * 2, &adj, &mut edges, &b);

    for j in 0..k_total {
        let u = edges[j * 2].tail;
        let w = edges[j * 2].head;
        if u <= n && w <= n * 2 && edges[j * 2].flow == 1 {
            ans[u][w - n] = 'X';
        }
    }

    for i in 1..=n {
        let line: String = ans[i][1..=n].iter().collect();
        print!("{line}\n");
    }
}
