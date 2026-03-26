use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Data {
    key: i64,
    id: usize,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key) // Reverse order for min-heap
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Data {}

#[derive(Debug, Clone)]
struct Edge {
    id: usize,
    head: usize,
    tail: usize,
    flow: i64,
    cap: i64,
    cost: i64,
}

#[derive(Debug, Clone)]
struct List {
    next: Option<Box<List>>,
    e: Edge,
}

const SUP: i64 = 1 << 60;
const INF: i64 = -SUP;

fn successive_shortest_path(n: usize, m: usize, adj: &Vec<Option<Box<List>>>, e: &mut Vec<Edge>, b: &mut Vec<i64>) -> i64 {
    let mut flag = vec![0; n + 1];
    let mut def = vec![0; n + 1];
    let mut ans = 0;
    let mut dist = vec![0; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev = vec![None; n + 1];
    let mut h_def = BinaryHeap::new();
    let mut h_dist = BinaryHeap::new();

    for i in 0..m {
        ans += e[i].cost * e[i].flow;
    }

    for i in 1..=n {
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
                for j in 1..=n {
                    let mut flag_0 = 0;
                    for k in 0..m {
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
                if j > n {
                    return INF;
                }
            }
        }
    }

    while let Some(d) = h_def.pop() {
        let s = d.id;
        for u in 1..=n {
            flag[u] = 0;
            dist[u] = SUP;
            prev[u] = None;
        }
        dist[s] = 0;
        h_dist.clear();
        h_dist.push(Data { key: 0, id: s });

        while let Some(d) = h_dist.pop() {
            let u = d.id;
            if flag[u] != 0 {
                continue;
            }
            flag[u] = 1;

            let mut p = &adj[u];
            while let Some(node) = p {
                let w = node.e.head;
                if node.e.flow == node.e.cap {
                    p = node.next.as_deref();
                    continue;
                }
                if dist[w] > dist[u] + node.e.cost + pi[u] - pi[w] {
                    dist[w] = dist[u] + node.e.cost + pi[u] - pi[w];
                    prev[w] = Some(node.e);
                    h_dist.push(Data { key: dist[w], id: w });
                }
                p = node.next.as_deref();
            }
        }

        let t = (1..=n).find(|&t| dist[t] != SUP && def[t] > 0).unwrap_or(n + 1);
        if t > n {
            return SUP;
        }

        let mut u = t;
        let mut min = -def[s].min(def[t]);
        while let Some(edge) = prev[u] {
            if min > edge.cap - edge.flow {
                min = edge.cap - edge.flow;
            }
            u = edge.tail;
        }
        u = t;
        while let Some(edge) = prev[u] {
            let i = edge.id;
            if e[i ^ 1].flow > 0 {
                e[i ^ 1].flow -= min;
                e[i].cap -= min;
            } else {
                e[i].flow += min;
                e[i ^ 1].cap += min;
            }
            ans += e[i].cost * min;
            u = edge.tail;
        }
        for i in 1..=n {
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = dimensions.next().unwrap();
    let m = dimensions.next().unwrap();

    let mut s = vec![vec![' '; m + 2]; n + 2];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            s[i][j + 1] = c;
        }
    }

    let mut dist = vec![-1; 2501];
    let mut q = vec![0; 2501];
    let mut adj: Vec<Option<Box<List>>> = vec![None; 2603];
    let mut d = vec![List { next: None, e: Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 } }; 30000];
    let mut e = vec![Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; 30000];
    let mut m = 0;
    let mut n = 1;

    for i in (1..=n).rev() {
        for j in (1..=m).rev() {
            if s[i][j] != 'o' {
                continue;
            }
            s[i][j] = '.';
            dist.fill(-1);
            let start = (i - 1) * m + j;
            dist[start] = 0;
            let (mut head, mut tail) = (0, 1);
            q[0] = start;

            while head < tail {
                let k = (q[head] - 1) / m + 1;
                let l = (q[head] - 1) % m + 1;
                if s[k + 1][l] == '.' && dist[k * m + l] == -1 {
                    dist[k * m + l] = dist[q[head]] + 1;
                    q[tail] = k * m + l;
                    tail += 1;
                }
                if s[k][l + 1] == '.' && dist[(k - 1) * m + l + 1] == -1 {
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
                d[m * 2].next = adj[n * m + n].take();
                d[m * 2 + 1].next = adj[q[head]].take();
                adj[n * m + n] = Some(Box::new(d[m * 2].clone()));
                adj[q[head]] = Some(Box::new(d[m * 2 + 1].clone()));
                m += 1;
            }
            n += 1;
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
        d[m * 2].next = adj[n * m + n].take();
        d[m * 2 + 1].next = adj[n * m + i].take();
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
        d[m * 2].next = adj[i].take();
        d[m * 2 + 1].next = adj[n * m + n + 1].take();
        adj[i] = Some(Box::new(d[m * 2].clone()));
        adj[n * m + n + 1] = Some(Box::new(d[m * 2 + 1].clone()));
        m += 1;
    }

    let mut b = vec![0; 2603];
    b[n * m + n] = -(n as i64 - 1);
    b[n * m + n + 1] = n as i64 - 1;

    println!("{}", -successive_shortest_path(n * m + n + 1, m * 2, &adj, &mut e, &mut b));
}
