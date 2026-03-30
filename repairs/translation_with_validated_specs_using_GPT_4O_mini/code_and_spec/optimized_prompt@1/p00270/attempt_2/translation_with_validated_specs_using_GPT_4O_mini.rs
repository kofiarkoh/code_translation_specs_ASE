use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const INF: usize = 0x10101010;

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: usize,
}

fn dijkstra(start: usize, graph: &[Vec<Edge>], dist: &mut [usize]) {
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((0, start)); // (distance, node)

    while let Some((d, u)) = heap.pop() {
        let d = d.wrapping_neg(); // Convert back to positive distance
        if d > dist[u] {
            continue;
        }

        for edge in &graph[u] {
            let v = edge.to;
            let new_dist = dist[u].wrapping_add(edge.weight);
            if new_dist < dist[v] {
                dist[v] = new_dist;
                heap.push((new_dist.wrapping_neg(), v));
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let s: usize = lines.next().unwrap().parse().unwrap();
    let r: usize = lines.next().unwrap().parse().unwrap();
    
    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); s];
    
    for _ in 0..r {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let weight: usize = parts.next().unwrap().parse::<usize>().unwrap();
        
        graph[u].push(Edge { to: v, weight });
        graph[v].push(Edge { to: u, weight });
    }

    let (a, b): (usize, usize) = {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        (parts.next().unwrap().parse::<usize>().unwrap() - 1, 
         parts.next().unwrap().parse::<usize>().unwrap() - 1)
    };

    let q: usize = lines.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(q);
    
    for _ in 0..q {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let c: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let d: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        queries.push((c, d));
    }

    let mut dist = vec![INF; s];
    dijkstra(a, &graph, &mut dist);

    let mut reachable = vec![0u64; (q + 63) / 64];
    let mut order = Vec::new();
    let mut visited = vec![false; s];
    let mut stack = vec![b];

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        order.push(node);
        for edge in &graph[node] {
            if dist[node] == dist[edge.to] + edge.weight {
                stack.push(edge.to);
            }
        }
    }

    let mut deg = vec![0; s];
    for &node in &order {
        for edge in &graph[node] {
            if dist[node] == dist[edge.to] + edge.weight {
                deg[edge.to] += 1;
            }
        }
    }

    let mut queue = vec![a];
    let mut ord = Vec::new();

    while let Some(node) = queue.pop() {
        ord.push(node);
        for edge in &graph[node] {
            if deg[edge.to] > 0 {
                deg[edge.to] -= 1;
                if deg[edge.to] == 0 {
                    queue.push(edge.to);
                }
            }
        }
    }

    for i in 0..reachable.len() {
        reachable[i] = 0;
        for j in 0..64 {
            if (i * 64 + j) < q {
                reachable[i] |= 1 << j;
            }
        }
        for &node in &ord {
            for edge in &graph[node] {
                reachable[edge.to] |= reachable[node];
            }
        }
        for j in 0..64 {
            if (i * 64 + j) < q {
                let c = queries[i * 64 + j].0;
                let d = queries[i * 64 + j].1;
                if reachable[d] & (1 << j) != 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
