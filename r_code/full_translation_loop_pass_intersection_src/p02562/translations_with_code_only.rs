use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Data {
    key: i64,
    id: i32,
}

struct MinHeap {
    // 1-based heap array, obj[0] unused
    obj: Vec<Data>,
    size: usize,
}

impl MinHeap {
    fn with_capacity(cap: usize) -> Self {
        // Fill with dummy; index 0 unused.
        Self {
            obj: vec![
                Data { key: 0, id: 0 };
                cap.max(1) + 1
            ],
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
            if j < self.size {
                // j^1 toggles between 2k and 2k+1
                let alt = j ^ 1;
                if self.obj[alt].key < self.obj[j].key {
                    j = alt;
                }
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

#[derive(Clone, Copy)]
struct Edge {
    id: i32,
    head: i32,
    tail: i32,
    flow: i32,
    cap: i32,
    cost: i32,
}

#[derive(Clone, Copy)]
struct ListNode {
    next: i32, // index into nodes vec, -1 for NULL
    e: i32,    // index into edges vec
}

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(
    n: usize,
    m: usize,
    adj: &Vec<i32>, // 1..=n head indices into nodes, -1 for NULL
    nodes: &Vec<ListNode>,
    edges: &mut Vec<Edge>,
    b: &Vec<i32>, // 1..=n
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
                    flag[0] = 0;
                    for k in 0..m {
                        if edges[k].flow == edges[k].cap {
                            continue;
                        }
                        let u = edges[k].tail as usize;
                        let w = edges[k].head as usize;
                        if pi[u] != SUP {
                            let cand = pi[u] + edges[k].cost as i64;
                            if pi[w] > cand {
                                flag[0] = 1;
                                pi[w] = cand;
                            }
                        }
                    }
                    if flag[0] == 0 {
                        j_done = j;
                        break;
                    }
                }
                if j_done == 0 {
                    // means loop didn't break early => j > N in C
                    return INF;
                }
            }
        }
    }

    while h_def.size > 0 {
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

        while h_dist.size > 0 {
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
            let rev = (edges[ei].id as usize) ^ 1;
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

    let n_in: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let k_in: i32 = it.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0i32; 51]; 51];
    for i in 1..=n_in {
        for j in 1..=n_in {
            a[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // Build graph using index-based adjacency lists to avoid unsafe pointers.
    // Max sizes follow the C arrays: d[6000], e[6000], adj[103]
    let mut adj = vec![-1i32; 103];
    let mut nodes: Vec<ListNode> = vec![
        ListNode { next: -1, e: -1 };
        6000
    ];
    let mut edges: Vec<Edge> = vec![
        Edge {
            id: 0,
            head: 0,
            tail: 0,
            flow: 0,
            cap: 0,
            cost: 0
        };
        6000
    ];

    let mut k: usize = 0;
    for u in 1..=n_in {
        for w in (n_in + 1)..=(n_in * 2) {
            let e0 = k * 2;
            let e1 = k * 2 + 1;
            edges[e0].id = e0 as i32;
            edges[e0].tail = u as i32;
            edges[e0].head = w as i32;
            edges[e0].cap = 1;
            edges[e0].cost = -a[u][w - n_in];

            edges[e1].id = e1 as i32;
            edges[e1].tail = w as i32;
            edges[e1].head = u as i32;
            edges[e1].cap = 0;
            edges[e1].cost = a[u][w - n_in];

            nodes[e0].e = e0 as i32;
            nodes[e1].e = e1 as i32;

            nodes[e0].next = adj[u];
            nodes[e1].next = adj[w];
            adj[u] = e0 as i32;
            adj[w] = e1 as i32;

            k += 1;
        }
    }

    for w in 1..=n_in {
        let u = n_in * 2 + 1;
        let e0 = k * 2;
        let e1 = k * 2 + 1;

        edges[e0].id = e0 as i32;
        edges[e0].tail = u as i32;
        edges[e0].head = w as i32;
        edges[e0].cap = k_in;
        edges[e0].cost = 0;

        edges[e1].id = e1 as i32;
        edges[e1].tail = w as i32;
        edges[e1].head = u as i32;
        edges[e1].cap = 0;
        edges[e1].cost = 0;

        nodes[e0].e = e0 as i32;
        nodes[e1].e = e1 as i32;

        nodes[e0].next = adj[u];
        nodes[e1].next = adj[w];
        adj[u] = e0 as i32;
        adj[w] = e1 as i32;

        k += 1;
    }

    for u in (n_in + 1)..=(n_in * 2) {
        let w = n_in * 2 + 2;
        let e0 = k * 2;
        let e1 = k * 2 + 1;

        edges[e0].id = e0 as i32;
        edges[e0].tail = u as i32;
        edges[e0].head = w as i32;
        edges[e0].cap = k_in;
        edges[e0].cost = 0;

        edges[e1].id = e1 as i32;
        edges[e1].tail = w as i32;
        edges[e1].head = u as i32;
        edges[e1].cap = 0;
        edges[e1].cost = 0;

        nodes[e0].e = e0 as i32;
        nodes[e1].e = e1 as i32;

        nodes[e0].next = adj[u];
        nodes[e1].next = adj[w];
        adj[u] = e0 as i32;
        adj[w] = e1 as i32;

        k += 1;
    }

    let n_total = n_in * 2 + 2;
    let m_total = k * 2;

    let mut b = vec![0i32; 103];
    let mut argmax: i32 = 0;
    let mut maxv: i64 = 0;

    for i in 1..=((n_in as i32) * k_in) {
        b[n_in * 2 + 1] = -i;
        b[n_in * 2 + 2] = i;
        let tmp = -succesive_shortest_path(n_total, m_total, &adj, &nodes, &mut edges, &b);
        if tmp != SUP && maxv < tmp {
            maxv = tmp;
            argmax = i;
        }
    }

    print!("{maxv}\n");

    let mut ans = vec![vec!['.'; 52]; 52];
    for i in 1..=n_in {
        for j in 1..=n_in {
            ans[i][j] = '.';
        }
    }

    b[n_in * 2 + 1] = -argmax;
    b[n_in * 2 + 2] = argmax;

    for j in 0..k {
        let e0 = j * 2;
        let e1 = j * 2 + 1;
        edges[e0].flow = 0;
        edges[e1].cap = 0;
    }

    succesive_shortest_path(n_total, m_total, &adj, &nodes, &mut edges, &b);

    for j in 0..k {
        let e0 = j * 2;
        let u = edges[e0].tail as usize;
        let w = edges[e0].head as usize;
        if u <= n_in && w <= n_in * 2 && edges[e0].flow == 1 {
            ans[u][w - n_in] = 'X';
        }
    }

    for i in 1..=n_in {
        let mut line = String::with_capacity(n_in);
        for j in 1..=n_in {
            line.push(ans[i][j]);
        }
        print!("{line}\n");
    }

    // Match C's fflush(stdout)
    io::Write::flush(&mut io::stdout()).unwrap();
}
