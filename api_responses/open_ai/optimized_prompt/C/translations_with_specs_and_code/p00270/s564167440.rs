use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const INF: i32 = 0x10101010;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

fn dijkstra(start: usize, graph: &[Vec<Edge>], dist: &mut Vec<i32>) {
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((0, start)); // (distance, node)

    while let Some((d, u)) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for edge in &graph[u] {
            let new_dist = dist[u] + edge.weight;
            if new_dist < dist[edge.to] {
                dist[edge.to] = new_dist;
                heap.push((new_dist, edge.to));
            }
        }
    }
}

fn topological_sort(start: usize, goal: usize, dist: &[i32], graph: &[Vec<Edge>]) -> Vec<usize> {
    let mut mk = vec![false; graph.len()];
    let mut to2 = vec![vec![]; graph.len()];
    let mut q = vec![goal];
    let mut cnt = 0;

    while let Some(s) = q.pop() {
        if mk[s] {
            continue;
        }
        for edge in &graph[s] {
            if dist[s] == dist[edge.to] + edge.weight {
                q.push(edge.to);
                to2[edge.to].push(s);
            }
        }
        mk[s] = true;
    }

    let mut deg = vec![0; graph.len()];
    for s in 0..graph.len() {
        for &e in &to2[s] {
            deg[e] += 1;
        }
    }

    let mut ord = vec![];
    q.push(start);
    while let Some(s) = q.pop() {
        ord.push(s);
        for &e in &to2[s] {
            if deg[e] > 0 {
                deg[e] -= 1;
                if deg[e] == 0 {
                    q.push(e);
                }
            }
        }
    }

    ord
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let s: usize = lines.next().unwrap().parse().unwrap();
    let r: usize = lines.next().unwrap().parse().unwrap();
    
    let mut graph = vec![vec![]; s];
    let mut memo = vec![];

    for _ in 0..r {
        let edge: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let u = edge[0] - 1;
        let v = edge[1] - 1;
        let weight = edge[2];
        graph[u].push(Edge { to: v, weight });
        graph[v].push(Edge { to: u, weight });
        memo.push((u, v, weight));
    }

    let a: usize = lines.next().unwrap().parse().unwrap() - 1;
    let b: usize = lines.next().unwrap().parse().unwrap() - 1;
    let q: usize = lines.next().unwrap().parse().unwrap();

    let mut queries = vec![];
    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        queries.push((query[0] - 1, query[1] - 1));
    }

    let mut dist = vec![INF; s];
    dijkstra(a, &graph, &mut dist);
    let ord = topological_sort(a, b, &dist, &graph);

    let mut dp = vec![0u64; s];
    let ii = (q + 63) / 64;

    for i in 0..ii {
        dp.fill(0);
        for j in 0..64 {
            if (i * 64 + j) < q {
                dp[queries[i * 64 + j].0] |= 1 << j;
            }
        }
        for &u in &ord {
            for edge in &graph[u] {
                dp[edge.to] |= dp[u];
            }
        }
        for j in 0..64 {
            if (i * 64 + j) < q {
                if dp[queries[i * 64 + j].1] & (1 << j) != 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
