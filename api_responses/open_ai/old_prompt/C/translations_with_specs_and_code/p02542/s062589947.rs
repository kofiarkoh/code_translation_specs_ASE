use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Data {
    key: i64,
    id: usize,
}

struct MinHeap {
    obj: Vec<Data>,
    size: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            obj: vec![Data { key: 0, id: 0 }; 1], // 1-indexed
            size: 0,
        }
    }

    fn push(&mut self, x: Data) {
        self.size += 1;
        let mut i = self.size;
        self.obj.push(x);
        while i > 1 {
            let j = i / 2;
            if self.obj[i].key < self.obj[j].key {
                self.obj.swap(i, j);
                i = j;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Data {
        let output = self.obj[1];
        self.obj[1] = self.obj[self.size];
        self.size -= 1;
        let mut i = 1;
        let mut j = 2;
        while j <= self.size {
            if j < self.size && self.obj[j + 1].key < self.obj[j].key {
                j += 1;
            }
            if self.obj[j].key < self.obj[i].key {
                self.obj.swap(i, j);
                i = j;
                j *= 2;
            } else {
                break;
            }
        }
        output
    }
}

#[derive(Clone)]
struct Edge {
    id: usize,
    head: usize,
    tail: usize,
    flow: i64,
    cap: i64,
    cost: i64,
}

struct List {
    next: Option<Box<List>>,
    e: Edge,
}

const SUP: i64 = 1 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(N: usize, M: usize, adj: &Vec<Option<Box<List>>>, e: &mut Vec<Edge>, b: &mut Vec<i64>) -> i64 {
    let mut flag = vec![0; N + 1];
    let mut def = vec![0; N + 1];
    let mut ans = 0;
    let mut dist = vec![0; N + 1];
    let mut pi = vec![SUP; N + 1];
    let mut prev: Vec<Option<usize>> = vec![None; N + 1];
    let mut h_def = MinHeap::new();
    let mut h_dist = MinHeap::new();

    for i in 0..M {
        ans += e[i].cost * e[i].flow;
    }

    for i in 1..=N {
        def[i] = b[i];
        let mut p = &adj[i];
        while let Some(node) = p {
            def[i] += node.e.flow - e[node.e.id ^ 1].flow;
            p = node.next.as_deref();
        }
        if def[i] < 0 {
            h_def.push(Data { key: def[i], id: i });
            if pi[i] == SUP {
                pi[i] = 0;
                for j in 1..=N {
                    let mut flag_0 = 0;
                    for k in 0..M {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost {
                            flag_0 = 1;
                            pi[w] = pi[u] + e[k].cost;
                        }
                    }
                    if flag_0 == 0 {
                        break;
                    }
                }
                if j > N {
                    return INF;
                }
            }
        }
    }

    let mut s;
    let mut t;
    let mut min;
    while h_def.size > 0 {
        let d = h_def.pop();
        s = d.id;
        for u in 1..=N {
            flag[u] = 0;
            dist[u] = SUP;
            prev[u] = None;
        }
        dist[s] = 0;
        h_dist.size = 0;
        h_dist.push(Data { key: 0, id: s });

        while h_dist.size > 0 {
            let d = h_dist.pop();
            let u = d.id;
            if flag[u] != 0 {
                continue;
            } else {
                flag[u] = 1;
            }

            let mut p = &adj[u];
            while let Some(node) = p {
                let w = node.e.head;
                if node.e.flow == node.e.cap {
                    continue;
                } else if dist[w] > dist[u] + node.e.cost + pi[u] - pi[w] {
                    dist[w] = dist[u] + node.e.cost + pi[u] - pi[w];
                    prev[w] = Some(node.e.id);
                    h_dist.push(Data { key: dist[w], id: w });
                }
                p = node.next.as_deref();
            }
        }

        t = (1..=N).find(|&t| dist[t] != SUP && def[t] > 0).unwrap_or(N + 1);
        if t > N {
            return SUP;
        }

        let mut u = t;
        min = (-def[s]).min(def[t]);
        while let Some(prev_id) = prev[u] {
            if min > e[prev_id].cap - e[prev_id].flow {
                min = e[prev_id].cap - e[prev_id].flow;
            }
            u = e[prev_id].tail;
        }
        u = t;
        while let Some(prev_id) = prev[u] {
            if e[prev_id ^ 1].flow > 0 {
                e[prev_id ^ 1].flow -= min;
                e[prev_id].cap -= min;
            } else {
                e[prev_id].flow += min;
                e[prev_id ^ 1].cap += min;
            }
            ans += e[prev_id].cost * min;
            u = e[prev_id].tail;
        }
        for i in 1..=N {
            if dist[i] != SUP {
                pi[i] += dist[i];
            }
        }

        def[s] += min;
        def[t] -= min;
        if def[s] < 0 {
            h_def.push(Data { key: def[s], id: s });
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut S = vec![vec![' '; m + 2]; n + 2];
    for i in 1..=n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        for j in 1..=m {
            S[i][j] = line.chars().nth(j - 1).unwrap();
        }
    }

    let mut dist = vec![-1; 2501];
    let mut q = vec![0; 2501];
    let mut adj: Vec<Option<Box<List>>> = vec![None; 2603];
    let mut d = vec![List { next: None, e: Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; }; 30000];
    let mut e = vec![Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; 30000];
    let mut m = 0;
    let mut n = 1;

    for i in (1..=n).rev() {
        for j in (1..=m).rev() {
            if S[i][j] != 'o' {
                continue;
            } else {
                S[i][j] = '.';
                for k in 1..=n * m {
                    dist[k] = -1;
                }
                let start = (i - 1) * m + j;
                dist[start] = 0;
                let mut head = 0;
                let mut tail = 1;
                q[0] = start;

                while head < tail {
                    let k = (q[head] - 1) / m + 1;
                    let l = (q[head] - 1) % m + 1;
                    if S[k + 1][l] == '.' && dist[k * m + l] == -1 {
                        dist[k * m + l] = dist[q[head]] + 1;
                        q[tail] = k * m + l;
                        tail += 1;
                    }
                    if S[k][l + 1] == '.' && dist[(k - 1) * m + l + 1] == -1 {
                        dist[(k - 1) * m + l + 1] = dist[q[head]] + 1;
                        q[tail] = (k - 1) * m + l + 1;
                        tail += 1;
                    }
                    head += 1;
                }

                head -= 1;
                while head >= 0 && head >= tail - 100 {
                    e[m * 2].id = m * 2;
                    e[m * 2].tail = n * m + n;
                    e[m * 2].head = q[head];
                    e[m * 2].cap = 1;
                    e[m * 2].cost = -dist[q[head]];
                    e[m * 2 + 1].id = m * 2 + 1;
                    e[m * 2 + 1].tail = q[head];
                    e[m * 2 + 1].head = n * m + n;
                    e[m * 2 + 1].cap = 0;
                    e[m * 2 + 1].cost = dist[q[head]];
                    d[m * 2].e = e[m * 2].clone();
                    d[m * 2 + 1].e = e[m * 2 + 1].clone();
                    d[m * 2].next = adj[n * m + n].take().map(Box::new);
                    d[m * 2 + 1].next = adj[q[head]].take().map(Box::new);
                    adj[n * m + n] = Some(Box::new(d[m * 2].clone()));
                    adj[q[head]] = Some(Box::new(d[m * 2 + 1].clone()));
                    m += 1;
                }
                n += 1;
            }
        }
    }

    for i in 1..n {
        e[m * 2].id = m * 2;
        e[m * 2].tail = n * m + n;
        e[m * 2].head = n * m + i;
        e[m * 2].cap = 1;
        e[m * 2].cost = 0;
        e[m * 2 + 1].id = m * 2 + 1;
        e[m * 2 + 1].tail = n * m + i;
        e[m * 2 + 1].head = n * m + n;
        e[m * 2 + 1].cap = 0;
        e[m * 2 + 1].cost = 0;
        d[m * 2].e = e[m * 2].clone();
        d[m * 2 + 1].e = e[m * 2 + 1].clone();
        d[m * 2].next = adj[n * m + n].take().map(Box::new);
        d[m * 2 + 1].next = adj[n * m + i].take().map(Box::new);
        adj[n * m + n] = Some(Box::new(d[m * 2].clone()));
        adj[n * m + i] = Some(Box::new(d[m * 2 + 1].clone()));
        m += 1;
    }

    for i in 1..=n * m {
        e[m * 2].id = m * 2;
        e[m * 2].tail = i;
        e[m * 2].head = n * m + n + 1;
        e[m * 2].cap = 1;
        e[m * 2].cost = 0;
        e[m * 2 + 1].id = m * 2 + 1;
        e[m * 2 + 1].tail = n * m + n + 1;
        e[m * 2 + 1].head = i;
        e[m * 2 + 1].cap = 0;
        e[m * 2 + 1].cost = 0;
        d[m * 2].e = e[m * 2].clone();
        d[m * 2 + 1].e = e[m * 2 + 1].clone();
        d[m * 2].next = adj[i].take().map(Box::new);
        d[m * 2 + 1].next = adj[n * m + n + 1].take().map(Box::new);
        adj[i] = Some(Box::new(d[m * 2].clone()));
        adj[n * m + n + 1] = Some(Box::new(d[m * 2 + 1].clone()));
        m += 1;
    }

    let mut b = vec![0; 2603];
    b[n * m + n] = -(n - 1) as i64;
    b[n * m + n + 1] = (n - 1) as i64;

    println!("{}", -succesive_shortest_path(n * m + n + 1, m * 2, &adj, &mut e, &mut b));
}
