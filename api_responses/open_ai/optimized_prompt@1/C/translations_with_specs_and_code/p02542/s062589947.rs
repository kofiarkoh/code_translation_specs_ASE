use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
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
    let mut ans = 0;
    let mut dist = vec![0; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut def = vec![0; n + 1];
    let mut prev = vec![None; n + 1];
    let mut flag = vec![0; n + 1];

    let mut h_def = BinaryHeap::new();
    let mut h_dist = BinaryHeap::new();

    for i in 1..=n {
        def[i] = b[i];
        if def[i] < 0 {
            h_def.push(Data { key: def[i] as i64, id: i });
            if pi[i] == SUP {
                pi[i] = 0;
                for j in 1..=n {
                    let mut updated = false;
                    for k in 0..m {
                        if e[k].flow == e[k].cap { continue; }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost {
                            updated = true;
                            pi[w] = pi[u] + e[k].cost;
                        }
                    }
                    if !updated { break; }
                }
                if j > n { return INF; }
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
            if flag[u] != 0 { continue; }
            flag[u] = 1;

            if let Some(p) = &adj[u] {
                let mut current = p;
                while let Some(node) = current {
                    let w = node.e.head;
                    if node.e.flow == node.e.cap { 
                        current = node.next.as_deref(); 
                        continue; 
                    }
                    if dist[w] > dist[u] + node.e.cost + pi[u] - pi[w] {
                        dist[w] = dist[u] + node.e.cost + pi[u] - pi[w];
                        prev[w] = Some(node.e.clone());
                        h_dist.push(Data { key: dist[w], id: w });
                    }
                    current = node.next.as_deref();
                }
            }
        }

        let mut t = 0;
        for i in 1..=n {
            if dist[i] != SUP && def[i] > 0 {
                t = i;
                break;
            }
        }
        if t == 0 { return SUP; }

        let mut min = (-def[s]).min(def[t]);
        let mut u = t;
        while let Some(p) = prev[u] {
            min = min.min(p.cap - p.flow);
            u = p.tail;
        }
        u = t;
        while let Some(p) = prev[u] {
            let i = p.id;
            if e[i ^ 1].flow > 0 {
                e[i ^ 1].flow -= min;
                e[i].cap -= min;
            } else {
                e[i].flow += min;
                e[i ^ 1].cap += min;
            }
            ans += (e[i].cost as i64) * (min as i64);
            u = p.tail;
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
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = dimensions.next().unwrap();
    let m = dimensions.next().unwrap();

    let mut grid = vec![vec![' '; m + 2]; n + 2];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j + 1] = c;
        }
    }

    let mut adj: Vec<Option<Box<List>>> = vec![None; n * m + 3];
    let mut e: Vec<Edge> = Vec::new();
    let mut d: Vec<Box<List>> = Vec::new();
    let mut b = vec![0; n * m + 3];
    let mut m = 0;
    let mut n = 1;

    for i in (1..=n).rev() {
        for j in (1..=m).rev() {
            if grid[i][j] != 'o' { continue; }
            grid[i][j] = '.';
            let mut dist = vec![-1; n * m + 1];
            let mut q = vec![0; n * m + 1];
            let mut head = 0;
            let mut tail = 1;
            q[0] = (i - 1) * m + j;
            dist[q[0]] = 0;

            while head < tail {
                let k = (q[head] - 1) / m + 1;
                let l = (q[head] - 1) % m + 1;
                head += 1;

                if grid[k + 1][l] == '.' && dist[k * m + l] == -1 {
                    dist[k * m + l] = dist[q[head - 1]] + 1;
                    q[tail] = k * m + l;
                    tail += 1;
                }
                if grid[k][l + 1] == '.' && dist[(k - 1) * m + l + 1] == -1 {
                    dist[(k - 1) * m + l + 1] = dist[q[head - 1]] + 1;
                    q[tail] = (k - 1) * m + l + 1;
                    tail += 1;
                }
            }

            for head in (0..head).rev().take(100) {
                e.push(Edge {
                    id: m * 2,
                    tail: n * m + n,
                    head: q[head],
                    cap: 1,
                    flow: 0,
                    cost: -dist[q[head]] as i32,
                });
                e.push(Edge {
                    id: m * 2 + 1,
                    tail: q[head],
                    head: n * m + n,
                    cap: 0,
                    flow: 0,
                    cost: dist[q[head]] as i32,
                });
                d.push(Box::new(List { next: adj[n * m + n].take(), e: e[m * 2].clone() }));
                adj[n * m + n] = Some(d[m * 2].clone());
                d.push(Box::new(List { next: adj[q[head]].take(), e: e[m * 2 + 1].clone() }));
                adj[q[head]] = Some(d[m * 2 + 1].clone());
                m += 1;
            }
            n += 1;
        }
    }

    for i in 1..n {
        e.push(Edge {
            id: m * 2,
            tail: n * m + n,
            head: n * m + i,
            cap: 1,
            flow: 0,
            cost: 0,
        });
        e.push(Edge {
            id: m * 2 + 1,
            tail: n * m + i,
            head: n * m + n,
            cap: 0,
            flow: 0,
            cost: 0,
        });
        d.push(Box::new(List { next: adj[n * m + n].take(), e: e[m * 2].clone() }));
        adj[n * m + n] = Some(d[m * 2].clone());
        d.push(Box::new(List { next: adj[n * m + i].take(), e: e[m * 2 + 1].clone() }));
        adj[n * m + i] = Some(d[m * 2 + 1].clone());
        m += 1;
    }

    for i in 1..=n * m {
        e.push(Edge {
            id: m * 2,
            tail: i,
            head: n * m + n + 1,
            cap: 1,
            flow: 0,
            cost: 0,
        });
        e.push(Edge {
            id: m * 2 + 1,
            tail: n * m + n + 1,
            head: i,
            cap: 0,
            flow: 0,
            cost: 0,
        });
        d.push(Box::new(List { next: adj[i].take(), e: e[m * 2].clone() }));
        adj[i] = Some(d[m * 2].clone());
        d.push(Box::new(List { next: adj[n * m + n + 1].take(), e: e[m * 2 + 1].clone() }));
        adj[n * m + n + 1] = Some(d[m * 2 + 1].clone());
        m += 1;
    }

    b[n * m + n] = -(n - 1) as i32;
    b[n * m + n + 1] = (n - 1) as i32;

    println!("{}", -successive_shortest_path(n * m + n + 1, m * 2, &adj, &mut e, &mut b));
}
