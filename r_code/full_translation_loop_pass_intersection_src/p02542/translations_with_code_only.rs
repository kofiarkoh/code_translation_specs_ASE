use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Data {
    key: i64,
    id: i32,
}

// 1-indexed min-heap by key (matches C behavior)
struct MinHeap {
    obj: Vec<Data>,
    size: usize,
}
impl MinHeap {
    fn with_capacity(cap: usize) -> Self {
        // index 0 unused
        Self {
            obj: vec![Data::default(); cap + 1],
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
    id: i32,
    head: i32,
    tail: i32,
    flow: i32,
    cap: i32,
    cost: i32,
}

#[derive(Clone, Copy, Default)]
struct ListNode {
    next: i32, // index into nodes vec, -1 for NULL
    e: i32,    // index into edges vec
}

const SUP: i64 = 1i64 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(
    n: usize,
    m: usize,
    adj: &Vec<i32>, // head indices into nodes, -1 for NULL; 1..=n used
    nodes: &Vec<ListNode>,
    edges: &mut Vec<Edge>,
    b: &Vec<i32>, // 1..=n used
) -> i64 {
    let mut flag = vec![0i32; n + 1];
    let mut def = vec![0i32; n + 1];
    let mut dist = vec![SUP; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev = vec![-1i32; n + 1]; // edge index, -1 for NULL

    let mut ans: i64 = 0;
    for i in 0..m {
        ans += (edges[i].cost as i64) * (edges[i].flow as i64);
    }

    for i in 1..=n {
        pi[i] = SUP;
    }

    let mut h_def = MinHeap::with_capacity(n + 1);
    let mut h_dist = MinHeap::with_capacity(m * 2 + 1);

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
                        if pi[u] != SUP && pi[w] > pi[u] + edges[k].cost as i64 {
                            flag[0] = 1;
                            pi[w] = pi[u] + edges[k].cost as i64;
                        }
                    }
                    if flag[0] == 0 {
                        j_done = j;
                        break;
                    }
                }
                if j_done == 0 {
                    // j > N in C
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
                let cand = dist[u] + edges[ei].cost as i64 + pi[u] - pi[w];
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

        let mut u2 = t;
        while prev[u2] != -1 {
            let i_edge = prev[u2] as usize;
            let rev = (edges[i_edge].id as usize) ^ 1;
            if edges[rev].flow > 0 {
                edges[rev].flow -= minv;
                edges[i_edge].cap -= minv;
            } else {
                edges[i_edge].flow += minv;
                edges[rev].cap += minv;
            }
            ans += (edges[i_edge].cost as i64) * (minv as i64);
            u2 = edges[i_edge].tail as usize;
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
    let m_in: usize = it.next().unwrap().parse().unwrap();

    // S[52][52] with 1-based indexing; keep a border of '\0' like C's zero-init.
    let mut s = vec![vec![b'\0'; 52]; 52];
    for i in 1..=n_in {
        let row = it.next().unwrap().as_bytes();
        for (j, &ch) in row.iter().enumerate() {
            s[i][1 + j] = ch;
        }
    }

    let mut m: usize = 0;
    let mut n: usize = 1;

    let mut dist = vec![-1i32; 2501];
    let mut q = vec![0i32; 2501];

    // adjacency heads: size 2603, 0 unused; -1 means NULL
    let mut adj = vec![-1i32; 2603];

    // Preallocate like C arrays
    let mut nodes = vec![ListNode::default(); 30000];
    let mut edges = vec![Edge::default(); 30000];

    for i in (1..=n_in).rev() {
        for j in (1..=m_in).rev() {
            if s[i][j] != b'o' {
                continue;
            }
            s[i][j] = b'.';

            for k in 1..=n_in * m_in {
                dist[k] = -1;
            }
            q[0] = ((i - 1) * m_in + j) as i32;
            dist[q[0] as usize] = 0;

            let mut head: i32 = 0;
            let mut tail: i32 = 1;
            while head < tail {
                let cur = q[head as usize] as usize;
                let k = (cur - 1) / m_in + 1;
                let l = (cur - 1) % m_in + 1;

                if s[k + 1][l] == b'.' && dist[k * m_in + l] == -1 {
                    dist[k * m_in + l] = dist[cur] + 1;
                    q[tail as usize] = (k * m_in + l) as i32;
                    tail += 1;
                }
                if s[k][l + 1] == b'.' && dist[(k - 1) * m_in + l + 1] == -1 {
                    dist[(k - 1) * m_in + l + 1] = dist[cur] + 1;
                    q[tail as usize] = ((k - 1) * m_in + l + 1) as i32;
                    tail += 1;
                }
                head += 1;
            }

            head -= 1;
            while head >= 0 && head >= tail - 100 {
                let qh = q[head as usize] as i32;
                let idx0 = m * 2;
                let idx1 = m * 2 + 1;

                edges[idx0].id = idx0 as i32;
                edges[idx0].tail = (n_in * m_in + n) as i32;
                edges[idx0].head = qh;
                edges[idx0].cap = 1;
                edges[idx0].cost = -dist[qh as usize];

                edges[idx1].id = idx1 as i32;
                edges[idx1].tail = qh;
                edges[idx1].head = (n_in * m_in + n) as i32;
                edges[idx1].cap = 0;
                edges[idx1].cost = dist[qh as usize];

                nodes[idx0].e = idx0 as i32;
                nodes[idx1].e = idx1 as i32;

                let a_tail = (n_in * m_in + n) as usize;
                nodes[idx0].next = adj[a_tail];
                adj[a_tail] = idx0 as i32;

                let a_qh = qh as usize;
                nodes[idx1].next = adj[a_qh];
                adj[a_qh] = idx1 as i32;

                m += 1;
                head -= 1;
            }

            n += 1;
        }
    }

    for i in 1..n {
        let idx0 = m * 2;
        let idx1 = m * 2 + 1;

        edges[idx0].id = idx0 as i32;
        edges[idx0].tail = (n_in * m_in + n) as i32;
        edges[idx0].head = (n_in * m_in + i) as i32;
        edges[idx0].cap = 1;
        edges[idx0].cost = 0;

        edges[idx1].id = idx1 as i32;
        edges[idx1].tail = (n_in * m_in + i) as i32;
        edges[idx1].head = (n_in * m_in + n) as i32;
        edges[idx1].cap = 0;
        edges[idx1].cost = 0;

        nodes[idx0].e = idx0 as i32;
        nodes[idx1].e = idx1 as i32;

        let a_tail = (n_in * m_in + n) as usize;
        nodes[idx0].next = adj[a_tail];
        adj[a_tail] = idx0 as i32;

        let a_i = (n_in * m_in + i) as usize;
        nodes[idx1].next = adj[a_i];
        adj[a_i] = idx1 as i32;

        m += 1;
    }

    for i in 1..=n_in * m_in {
        let idx0 = m * 2;
        let idx1 = m * 2 + 1;

        edges[idx0].id = idx0 as i32;
        edges[idx0].tail = i as i32;
        edges[idx0].head = (n_in * m_in + n + 1) as i32;
        edges[idx0].cap = 1;
        edges[idx0].cost = 0;

        edges[idx1].id = idx1 as i32;
        edges[idx1].tail = (n_in * m_in + n + 1) as i32;
        edges[idx1].head = i as i32;
        edges[idx1].cap = 0;
        edges[idx1].cost = 0;

        nodes[idx0].e = idx0 as i32;
        nodes[idx1].e = idx1 as i32;

        let a_i = i as usize;
        nodes[idx0].next = adj[a_i];
        adj[a_i] = idx0 as i32;

        let a_sink = (n_in * m_in + n + 1) as usize;
        nodes[idx1].next = adj[a_sink];
        adj[a_sink] = idx1 as i32;

        m += 1;
    }

    let total_nodes = n_in * m_in + n + 1;
    let total_edges = m * 2;

    let mut b = vec![0i32; 2603];
    b[n_in * m_in + n] = -((n - 1) as i32);
    b[n_in * m_in + n + 1] = (n - 1) as i32;

    let res = succesive_shortest_path(total_nodes, total_edges, &adj, &nodes, &mut edges, &b);
    // Preserve exact output formatting
    print!("{}\n", -res);
}
