use std::collections::VecDeque;
use std::io::{self, BufRead};

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
            let j = i >> 1;
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
                j <<= 1;
            } else {
                break;
            }
        }
        output
    }
}

struct Edge {
    id: usize,
    head: usize,
    tail: usize,
    flow: i32,
    cap: i32,
    cost: i32,
}

struct List {
    next: Option<Box<List>>,
    e: Edge,
}

const SUP: i64 = 1 << 60;
const INF: i64 = -SUP;

fn successive_shortest_path(n: usize, m: usize, adj: &Vec<Option<Box<List>>>, e: &mut Vec<Edge>, b: &mut Vec<i32>) -> i64 {
    let mut flag = vec![0; n + 1];
    let mut def = vec![0; n + 1];
    let mut dist = vec![0; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut prev = vec![None; n + 1];
    let mut ans = 0;

    let mut h_def = MinHeap::new();
    let mut h_dist = MinHeap::new();

    for i in 0..m {
        ans += e[i].cost as i64 * e[i].flow as i64;
    }

    for i in 1..=n {
        def[i] = b[i];
        for p in &adj[i] {
            let mut current = p;
            while let Some(ref node) = current {
                def[i] += node.e.flow - e[node.e.id ^ 1].flow;
                current = node.next.as_deref();
            }
        }
        if def[i] < 0 {
            h_def.push(Data { key: def[i] as i64, id: i });
            if pi[i] == SUP {
                pi[i] = 0;
                for j in 1..=n {
                    let mut flag_updated = false;
                    for k in 0..m {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost as i64 {
                            flag_updated = true;
                            pi[w] = pi[u] + e[k].cost as i64;
                        }
                    }
                    if !flag_updated {
                        break;
                    }
                }
                if j > n {
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

        for u in 1..=n {
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
            }
            flag[u] = 1;

            if let Some(p) = &adj[u] {
                let mut current = p;
                while let Some(ref node) = current {
                    let w = node.e.head;
                    if node.e.flow == node.e.cap {
                        current = node.next.as_deref();
                        continue;
                    }
                    if dist[w] > dist[u] + node.e.cost as i64 + pi[u] - pi[w] {
                        dist[w] = dist[u] + node.e.cost as i64 + pi[u] - pi[w];
                        prev[w] = Some(node.e.clone());
                        h_dist.push(Data { key: dist[w], id: w });
                    }
                    current = node.next.as_deref();
                }
            }
        }

        t = (1..=n).find(|&t| dist[t] != SUP && def[t] > 0).unwrap_or(n + 1);
        if t > n {
            return SUP;
        }

        min = (-def[s]).min(def[t]);
        let mut u = t;
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
            ans += e[i].cost as i64 * min as i64;
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
            h_def.push(Data { key: def[s] as i64, id: s });
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![' '; m + 2]; n + 2];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for j in 1..=m {
            grid[i][j] = line.chars().nth(j - 1).unwrap();
        }
    }

    let mut adj: Vec<Option<Box<List>>> = vec![None; n * m + 3];
    let mut e: Vec<Edge> = Vec::new();
    let mut d: Vec<List> = Vec::new();
    let mut b = vec![0; n * m + 3];
    let mut m_count = 0;
    let mut n_count = 1;

    for i in (1..=n).rev() {
        for j in (1..=m).rev() {
            if grid[i][j] != 'o' {
                continue;
            }
            grid[i][j] = '.';
            let mut dist = vec![-1; n * m + 1];
            let mut q = VecDeque::new();
            let start = (i - 1) * m + j;
            dist[start] = 0;
            q.push_back(start);

            while let Some(head) = q.pop_front() {
                let k = (head - 1) / m + 1;
                let l = (head - 1) % m + 1;

                if grid[k + 1][l] == '.' && dist[k * m + l] == -1 {
                    dist[k * m + l] = dist[head] + 1;
                    q.push_back(k * m + l);
                }
                if grid[k][l + 1] == '.' && dist[(k - 1) * m + l + 1] == -1 {
                    dist[(k - 1) * m + l + 1] = dist[head] + 1;
                    q.push_back((k - 1) * m + l + 1);
                }
            }

            for head in (0..q.len()).rev().take(100) {
                let head_index = q[head];
                e.push(Edge {
                    id: m_count * 2,
                    tail: n * m + n_count,
                    head: head_index,
                    flow: 0,
                    cap: 1,
                    cost: -dist[head_index] as i32,
                });
                e.push(Edge {
                    id: m_count * 2 + 1,
                    tail: head_index,
                    head: n * m + n_count,
                    flow: 0,
                    cap: 0,
                    cost: dist[head_index] as i32,
                });
                d.push(List {
                    e: e[m_count * 2].clone(),
                    next: adj[n * m + n_count].take(),
                });
                adj[n * m + n_count] = Some(Box::new(d[m_count].clone()));
                d.push(List {
                    e: e[m_count * 2 + 1].clone(),
                    next: adj[head_index].take(),
                });
                adj[head_index] = Some(Box::new(d[m_count + 1].clone()));
                m_count += 1;
            }
            n_count += 1;
        }
    }

    for i in 1..=n_count {
        e.push(Edge {
            id: m_count * 2,
            tail: n * m + n_count,
            head: n * m + i,
            flow: 0,
            cap: 1,
            cost: 0,
        });
        e.push(Edge {
            id: m_count * 2 + 1,
            tail: n * m + i,
            head: n * m + n_count,
            flow: 0,
            cap: 0,
            cost: 0,
        });
        d.push(List {
            e: e[m_count * 2].clone(),
            next: adj[n * m + n_count].take(),
        });
        adj[n * m + n_count] = Some(Box::new(d[m_count].clone()));
        d.push(List {
            e: e[m_count * 2 + 1].clone(),
            next: adj[n * m + i].take(),
        });
        adj[n * m + i] = Some(Box::new(d[m_count + 1].clone()));
        m_count += 1;
    }

    for i in 1..=n * m {
        e.push(Edge {
            id: m_count * 2,
            tail: i,
            head: n * m + n_count + 1,
            flow: 0,
            cap: 1,
            cost: 0,
        });
        e.push(Edge {
            id: m_count * 2 + 1,
            tail: n * m + n_count + 1,
            head: i,
            flow: 0,
            cap: 0,
            cost: 0,
        });
        d.push(List {
            e: e[m_count * 2].clone(),
            next: adj[i].take(),
        });
        adj[i] = Some(Box::new(d[m_count].clone()));
        d.push(List {
            e: e[m_count * 2 + 1].clone(),
            next: adj[n * m + n_count + 1].take(),
        });
        adj[n * m + n_count + 1] = Some(Box::new(d[m_count + 1].clone()));
        m_count += 1;
    }

    b[n * m + n_count] = -(n_count - 1) as i32;
    b[n * m + n_count + 1] = (n_count - 1) as i32;

    let result = -successive_shortest_path(n * m + n_count + 1, m_count * 2, &adj, &mut e, &mut b);
    println!("{}", result);
}
