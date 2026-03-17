use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    node: usize,
    weight: u64,
}

#[derive(Debug)]
struct Graph {
    adjacency_list: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: u64) {
        self.adjacency_list[u].push(Edge { node: v, weight });
        self.adjacency_list[v].push(Edge { node: u, weight }); // Undirected graph
    }

    fn dijkstra(&self, start: usize) -> Vec<u64> {
        let mut dist = vec![u64::MAX; self.adjacency_list.len()];
        dist[start] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((0, start)); // (distance, node)

        while let Some((current_dist, current_node)) = heap.pop() {
            if current_dist > dist[current_node] {
                continue; // Skip if we have already found a better path
            }

            for edge in &self.adjacency_list[current_node] {
                let next = edge.node;
                let weight = edge.weight;
                let next_dist = current_dist + weight;

                if next_dist < dist[next] {
                    dist[next] = next_dist;
                    heap.push((next_dist, next));
                }
            }
        }

        dist
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let (s, r, q) = (params[0], params[1], params[2]);
    let mut graph = Graph::new(s);

    for _ in 0..r {
        let edge_line = lines.next().unwrap().unwrap();
        let edge_params: Vec<usize> = edge_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (u, v, w) = (edge_params[0], edge_params[1], edge_params[2] as u64);
        graph.add_edge(u, v, w);
    }

    let dist = graph.dijkstra(0); // Assuming we start from station 0

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let query_params: Vec<usize> = query_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (c, d) = (query_params[0], query_params[1]);

        // Check if there is a path from c to d
        if dist[d] < u64::MAX {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
