use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const INF: i32 = 0x10101010;

#[derive(Debug)]
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

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let s = params.next().unwrap();
    let r = params.next().unwrap();

    let mut graph = vec![Vec::new(); s];
    let mut hi = vec![0; s];

    let mut memo = Vec::with_capacity(r * 3);
    for _ in 0..r {
        let line = lines.next().unwrap().unwrap();
        let mut edge_data = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let u = edge_data.next().unwrap() - 1;
        let v = edge_data.next().unwrap() - 1;
        let weight = edge_data.next().unwrap() as i32;

        graph[u].push(Edge { to: v, weight });
        graph[v].push(Edge { to: u, weight });
        hi[u] += 1;
        hi[v] += 1;
    }

    let last_line = lines.next().unwrap().unwrap();
    let mut query_data = last_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let a = query_data.next().unwrap() - 1;
    let b = query_data.next().unwrap() - 1;
    let q = query_data.next().unwrap();

    let mut dist = vec![INF; s];
    dijkstra(a, &graph, &mut dist);

    let mut reachable = vec![0u64; s];
    reachable[a] |= 1 << 0;

    for i in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut query = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let c = query.next().unwrap() - 1;
        let d = query.next().unwrap() - 1;

        if dist[c] < INF {
            reachable[c] |= 1 << i;
        }

        if reachable[d] & (1 << i) != 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
