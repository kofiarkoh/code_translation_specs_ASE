use std::cmp;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Data {
    key: i64,
    id: usize,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.key.cmp(&self.key) // Reverse order for min-heap
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
    let mut dist = vec![SUP; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut def = vec![0; n + 1];
    let mut flag = vec![0; n + 1];
    let mut prev = vec![None; n + 1];
    let mut h_def = BinaryHeap::new();
    let mut h_dist = BinaryHeap::new();

    for edge in e.iter() {
        ans += edge.cost as i64 * edge.flow as i64;
    }

    for i in 1..=n {
        pi[i] = SUP;
    }

    for i in 1..=n {
        def[i] = b[i];
        if let Some(p) = &adj[i] {
            let mut current = p;
            while let Some(node) = current {
                def[i] += node.e.flow - e[node.e.id ^ 1].flow;
                current = node.next.as_deref();
            }
        }
        if def[i] < 0 {
            h_def.push(Data { key: def[i] as i64, id: i });
            if pi[i] == SUP {
                pi[i] = 0;
                for j in 1..=n {
                    let mut changed = false;
                    for k in 0..m {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost as i64 {
                            changed = true;
                            pi[w] = pi[u] + e[k].cost as i64;
                        }
                    }
                    if !changed {
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

            if let Some(p) = &adj[u] {
                let mut current = p;
                while let Some(node) = current {
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

        let mut t = 0;
        for i in 1..=n {
            if dist[i] != SUP && def[i] > 0 {
                t = i;
                break;
            }
        }
        if t == 0 {
            return SUP;
        }

        let mut min = cmp::min(-def[s], def[t]);
        let mut u = t;
        while let Some(p) = prev[u] {
            if min > p.cap - p.flow {
                min = p.cap - p.flow;
            }
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
            ans += e[i].cost as i64 * min as i64;
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
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        let costs: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for j in 1..=n {
            a[i][j] = costs[j - 1];
        }
    }

    let mut adj: Vec<Option<Box<List>>> = vec![None; 103];
    let mut d = vec![List { next: None, e: Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; }; 6000];
    let mut e = vec![Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; 6000];

    let mut k_index = 0;
    for u in 1..=n {
        for w in (n + 1)..=(n * 2) {
            e[k_index * 2].id = k_index * 2;
            e[k_index * 2].tail = u;
            e[k_index * 2].head = w;
            e[k_index * 2].cap = 1;
            e[k_index * 2].cost = -a[u][w - n];
            e[k_index * 2 + 1].id = k_index * 2 + 1;
            e[k_index * 2 + 1].tail = w;
            e[k_index * 2 + 1].head = u;
            e[k_index * 2 + 1].cap = 0;
            e[k_index * 2 + 1].cost = a[u][w - n];

            d[k_index * 2].e = e[k_index * 2].clone();
            d[k_index * 2 + 1].e = e[k_index * 2 + 1].clone();
            d[k_index * 2].next = adj[u].take().map(Box::new);
            d[k_index * 2 + 1].next = adj[w].take().map(Box::new);
            adj[u] = Some(Box::new(d[k_index * 2].clone()));
            adj[w] = Some(Box::new(d[k_index * 2 + 1].clone()));
            k_index += 1;
        }
    }

    for w in 1..=n {
        let u = n * 2 + 1;
        e[k_index * 2].id = k_index * 2;
        e[k_index * 2].tail = u;
        e[k_index * 2].head = w;
        e[k_index * 2].cap = k;
        e[k_index * 2].cost = 0;
        e[k_index * 2 + 1].id = k_index * 2 + 1;
        e[k_index * 2 + 1].tail = w;
        e[k_index * 2 + 1].head = u;
        e[k_index * 2 + 1].cap = 0;
        e[k_index * 2 + 1].cost = 0;

        d[k_index * 2].e = e[k_index * 2].clone();
        d[k_index * 2 + 1].e = e[k_index * 2 + 1].clone();
        d[k_index * 2].next = adj[u].take().map(Box::new);
        d[k_index * 2 + 1].next = adj[w].take().map(Box::new);
        adj[u] = Some(Box::new(d[k_index * 2].clone()));
        adj[w] = Some(Box::new(d[k_index * 2 + 1].clone()));
        k_index += 1;
    }

    for u in (n + 1)..=(n * 2) {
        let w = n * 2 + 2;
        e[k_index * 2].id = k_index * 2;
        e[k_index * 2].tail = u;
        e[k_index * 2].head = w;
        e[k_index * 2].cap = k;
        e[k_index * 2].cost = 0;
        e[k_index * 2 + 1].id = k_index * 2 + 1;
        e[k_index * 2 + 1].tail = w;
        e[k_index * 2 + 1].head = u;
        e[k_index * 2 + 1].cap = 0;
        e[k_index * 2 + 1].cost = 0;

        d[k_index * 2].e = e[k_index * 2].clone();
        d[k_index * 2 + 1].e = e[k_index * 2 + 1].clone();
        d[k_index * 2].next = adj[u].take().map(Box::new);
        d[k_index * 2 + 1].next = adj[w].take().map(Box::new);
        adj[u] = Some(Box::new(d[k_index * 2].clone()));
        adj[w] = Some(Box::new(d[k_index * 2 + 1].clone()));
        k_index += 1;
    }

    let mut b = vec![0; 103];
    let mut argmax = 0;
    let mut max = 0;

    for i in 1..=(n * k) {
        b[n * 2 + 1] = -i;
        b[n * 2 + 2] = i;
        let tmp = -successive_shortest_path(n * 2 + 2, k_index * 2, &adj, &mut e, &mut b);
        if tmp != SUP && max < tmp {
            max = tmp;
            argmax = i;
        }
    }

    println!("{}", max);

    let mut ans = vec![vec!['.'; n + 1]; n + 1];
    b[n * 2 + 1] = -argmax;
    b[n * 2 + 2] = argmax;

    for j in 0..k_index {
        e[j * 2].flow = 0;
        e[j * 2 + 1].cap = 0;
    }

    successive_shortest_path(n * 2 + 2, k_index * 2, &adj, &mut e, &mut b);

    for j in 0..k_index {
        let u = e[j * 2].tail;
        let w = e[j * 2].head;
        if u <= n && w <= n * 2 && e[j * 2].flow == 1 {
            ans[u][w - n] = 'X';
        }
    }

    for i in 1..=n {
        println!("{}", ans[i][1..].iter().collect::<String>());
    }
}
