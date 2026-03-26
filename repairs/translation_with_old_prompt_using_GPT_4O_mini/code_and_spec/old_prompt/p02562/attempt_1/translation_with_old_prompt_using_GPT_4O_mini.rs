use std::cmp;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Data {
    key: i64,
    id: usize,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Data {}

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

#[derive(Debug)]
struct Edge {
    id: usize,
    head: usize,
    tail: usize,
    flow: i64,
    cap: i64,
    cost: i64,
}

#[derive(Debug)]
struct List {
    next: Option<Box<List>>,
    e: Edge,
}

const SUP: i64 = 1 << 60;
const INF: i64 = -SUP;

fn successive_shortest_path(n: usize, m: usize, adj: &Vec<Option<Box<List>>>, e: &mut Vec<Edge>, b: &mut Vec<i64>) -> i64 {
    let mut ans = 0;
    let mut def = vec![0; n + 1];
    let mut pi = vec![SUP; n + 1];
    let mut dist = vec![0; n + 1];
    let mut prev = vec![None; n + 1];

    for edge in e.iter() {
        ans += edge.cost * edge.flow;
    }

    for i in 1..=n {
        def[i] = b[i];
        if let Some(p) = &adj[i] {
            let mut current = p;
            while let Some(next) = current {
                def[i] += next.e.flow - e[next.e.id ^ 1].flow;
                current = next.next.as_deref();
            }
        }
        if def[i] < 0 {
            let mut heap = BinaryHeap::new();
            heap.push(Data { key: def[i], id: i });
            if pi[i] == SUP {
                pi[i] = 0;
                for _ in 1..=n {
                    let mut flag = false;
                    for k in 0..m {
                        if e[k].flow == e[k].cap {
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost {
                            flag = true;
                            pi[w] = pi[u] + e[k].cost;
                        }
                    }
                    if !flag {
                        break;
                    }
                }
                if pi.iter().any(|&x| x == SUP) {
                    return INF;
                }
            }
        }
    }

    while let Some(d) = pop_min_heap(&mut heap) {
        let s = d.id;
        let mut flag = vec![0; n + 1];
        dist.fill(SUP);
        prev.fill(None);
        dist[s] = 0;

        let mut h_dist = BinaryHeap::new();
        h_dist.push(Data { key: 0, id: s });

        while let Some(d) = pop_min_heap(&mut h_dist) {
            let u = d.id;
            if flag[u] != 0 {
                continue;
            }
            flag[u] = 1;

            if let Some(p) = &adj[u] {
                let mut current = p;
                while let Some(next) = current {
                    let w = next.e.head;
                    if next.e.flow == next.e.cap {
                        current = next.next.as_deref();
                        continue;
                    }
                    if dist[w] > dist[u] + next.e.cost + pi[u] - pi[w] {
                        dist[w] = dist[u] + next.e.cost + pi[u] - pi[w];
                        prev[w] = Some(next.e);
                        h_dist.push(Data { key: dist[w], id: w });
                    }
                    current = next.next.as_deref();
                }
            }
        }

        let t = (1..=n).find(|&t| dist[t] != SUP && def[t] > 0).unwrap_or(0);
        if t == 0 {
            return SUP;
        }

        let min_flow = cmp::min(-def[s], def[t]);
        let mut u = t;
        while let Some(edge) = prev[u] {
            if min_flow > edge.cap - edge.flow {
                min_flow = edge.cap - edge.flow;
            }
            u = edge.tail;
        }

        u = t;
        while let Some(edge) = prev[u] {
            let i = edge.id;
            if e[i ^ 1].flow > 0 {
                e[i ^ 1].flow -= min_flow;
                e[i].cap -= min_flow;
            } else {
                e[i].flow += min_flow;
                e[i ^ 1].cap += min_flow;
            }
            ans += e[i].cost * min_flow;
            u = edge.tail;
        }

        for i in 1..=n {
            if dist[i] != SUP {
                pi[i] += dist[i];
            }
        }

        def[s] += min_flow;
        def[t] -= min_flow;
        if def[s] < 0 {
            heap.push(Data { key: def[s], id: s });
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
    let k: i64 = parts.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0; 52]; 52];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        let costs: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for j in 1..=n {
            a[i][j] = costs[j - 1];
        }
    }

    let mut adj: Vec<Option<Box<List>>> = vec![None; 103];
    let mut d = vec![List { next: None, e: Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; }; 6000];
    let mut e = vec![Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; 6000];

    let mut k = 0;
    for u in 1..=n {
        for w in n + 1..=n * 2 {
            e[k * 2] = Edge { id: k * 2, tail: u, head: w, cap: 1, cost: -a[u][w - n], flow: 0 };
            e[k * 2 + 1] = Edge { id: k * 2 + 1, tail: w, head: u, cap: 0, cost: a[u][w - n], flow: 0 };
            d[k * 2].e = e[k * 2].clone();
            d[k * 2 + 1].e = e[k * 2 + 1].clone();
            d[k * 2].next = adj[u].take().map(Box::new);
            d[k * 2 + 1].next = adj[w].take().map(Box::new);
            adj[u] = Some(Box::new(d[k * 2].clone()));
            adj[w] = Some(Box::new(d[k * 2 + 1].clone()));
            k += 1;
        }
    }

    for w in 1..=n {
        e[k * 2] = Edge { id: k * 2, tail: n * 2 + 1, head: w, cap: k, cost: 0, flow: 0 };
        e[k * 2 + 1] = Edge { id: k * 2 + 1, tail: w, head: n * 2 + 1, cap: 0, cost: 0, flow: 0 };
        d[k * 2].e = e[k * 2].clone();
        d[k * 2 + 1].e = e[k * 2 + 1].clone();
        d[k * 2].next = adj[n * 2 + 1].take().map(Box::new);
        d[k * 2 + 1].next = adj[w].take().map(Box::new);
        adj[n * 2 + 1] = Some(Box::new(d[k * 2].clone()));
        adj[w] = Some(Box::new(d[k * 2 + 1].clone()));
        k += 1;
    }

    for u in n + 1..=n * 2 {
        e[k * 2] = Edge { id: k * 2, tail: u, head: n * 2 + 2, cap: k, cost: 0, flow: 0 };
        e[k * 2 + 1] = Edge { id: k * 2 + 1, tail: n * 2 + 2, head: u, cap: 0, cost: 0, flow: 0 };
        d[k * 2].e = e[k * 2].clone();
        d[k * 2 + 1].e = e[k * 2 + 1].clone();
        d[k * 2].next = adj[u].take().map(Box::new);
        d[k * 2 + 1].next = adj[n * 2 + 2].take().map(Box::new);
        adj[u] = Some(Box::new(d[k * 2].clone()));
        adj[n * 2 + 2] = Some(Box::new(d[k * 2 + 1].clone()));
        k += 1;
    }

    let mut b = vec![0; 103];
    let mut argmax = 0;
    let mut max = 0;

    for i in 1..=n * k {
        b[n * 2 + 1] = -i;
        b[n * 2 + 2] = i;
        let tmp = -successive_shortest_path(n * 2 + 2, k * 2, &adj, &mut e, &mut b);
        if tmp != SUP && max < tmp {
            max = tmp;
            argmax = i;
        }
    }

    println!("{}", max);

    let mut ans = vec![vec!['.'; 52]; 52];
    for i in 1..=n {
        for j in 1..=n {
            ans[i][j] = '.';
        }
    }

    b[n * 2 + 1] = -argmax;
    b[n * 2 + 2] = argmax;

    for j in 0..k {
        e[j * 2].flow = 0;
        e[j * 2 + 1].cap = 0;
    }

    successive_shortest_path(n * 2 + 2, k * 2, &adj, &mut e, &mut b);

    for j in 0..k {
        let u = e[j * 2].tail;
        let w = e[j * 2].head;
        if u <= n && w <= n * 2 && e[j * 2].flow == 1 {
            ans[u][w - n] = 'X';
        }
    }

    for i in 1..=n {
        println!("{}", ans[i].iter().collect::<String>());
    }
}

fn pop_min_heap(heap: &mut BinaryHeap<Data>) -> Option<Data> {
    heap.pop()
}
