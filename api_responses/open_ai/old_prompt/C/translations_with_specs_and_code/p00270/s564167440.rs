// AOJ 0275 Railroad
// 2018.2.21 bal4u

use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const QMAX: usize = 200_000;
const INF: usize = 0x10101010;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    weight: usize,
    node: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight) // Reverse order for min-heap
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let s: usize = params.next().unwrap().parse().unwrap();
    let r: usize = params.next().unwrap().parse().unwrap();

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; s];
    let mut hi = vec![0; s];

    for _ in 0..r {
        let line = lines.next().unwrap().unwrap();
        let mut edge_data = line.split_whitespace();
        let u: usize = edge_data.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = edge_data.next().unwrap().parse::<usize>().unwrap() - 1;
        let weight: usize = edge_data.next().unwrap().parse::<usize>().unwrap();

        graph[u].push((v, weight));
        graph[v].push((u, weight));
        hi[u] += 1;
        hi[v] += 1;
    }

    let query_line = lines.next().unwrap().unwrap();
    let mut query_data = query_line.split_whitespace();
    let a: usize = query_data.next().unwrap().parse::<usize>().unwrap() - 1;
    let b: usize = query_data.next().unwrap().parse::<usize>().unwrap() - 1;
    let q: usize = query_data.next().unwrap().parse::<usize>().unwrap();

    let mut c = vec![0; q];
    let mut d = vec![0; q];

    for i in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut query = line.split_whitespace();
        c[i] = query.next().unwrap().parse::<usize>().unwrap() - 1;
        d[i] = query.next().unwrap().parse::<usize>().unwrap() - 1;
    }

    let dist = dijkstra(&graph, a);
    let ord = tsort(&graph, &dist, b);

    let mut dp = vec![0u64; s];
    let ii = (q + 63) >> 6;

    for i in 0..ii {
        dp.fill(0);
        for j in 0..64 {
            if (i << 6) + j < q {
                dp[c[(i << 6) + j]] |= 1 << j;
            }
        }
        for &u in &ord {
            for &(v, _) in &graph[u] {
                dp[v] |= dp[u];
            }
        }
        for j in 0..64 {
            if (i << 6) + j < q {
                if dp[d[(i << 6) + j]] & (1 << j) != 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![INF; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(Edge { weight: 0, node: start });

    while let Some(Edge { weight, node }) = heap.pop() {
        if weight > dist[node] {
            continue;
        }
        for &(neighbor, edge_weight) in &graph[node] {
            let new_dist = weight + edge_weight;
            if new_dist < dist[neighbor] {
                dist[neighbor] = new_dist;
                heap.push(Edge { weight: new_dist, node: neighbor });
            }
        }
    }
    dist
}

fn tsort(graph: &Vec<Vec<(usize, usize)>>, dist: &Vec<usize>, goal: usize) -> Vec<usize> {
    let mut mk = vec![false; graph.len()];
    let mut q = vec![goal];
    let mut top = 0;
    let mut tail = 1;
    let mut to2: Vec<Vec<usize>> = vec![vec![]; graph.len()];
    let mut hi2 = vec![0; graph.len()];

    while top < tail {
        let s = q[top];
        top += 1;
        if mk[s] {
            continue;
        }
        for &(e, weight) in &graph[s] {
            if dist[s] == dist[e] + weight {
                q.push(e);
                to2[e].push(s);
                hi2[e] += 1;
            }
        }
        mk[s] = true;
    }

    let mut ord = vec![];
    let mut deg = vec![0; graph.len()];

    for s in 0..graph.len() {
        for &e in &to2[s] {
            deg[e] += 1;
        }
    }

    q.clear();
    q.push(0);
    top = 0;
    tail = 1;

    while top < tail {
        let s = q[top];
        top += 1;
        ord.push(s);
        for &e in &to2[s] {
            if deg[e] > 0 {
                deg[e] -= 1;
                if deg[e] == 0 {
                    q.push(e);
                    tail += 1;
                }
            }
        }
    }
    ord
}
