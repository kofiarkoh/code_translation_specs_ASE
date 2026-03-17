use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.edges[from].push(Edge { to, weight });
    }
}

#[derive(Debug)]
struct Node {
    cost: i64,
    index: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a max-heap, so we reverse the ordering
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

fn modified_shortest_path(n: usize, m: usize, k: usize, costs: Vec<i64>, edges: Vec<(usize, usize, i32)>) {
    let mut graph = Graph::new(n);
    for (a, b, d) in edges {
        graph.add_edge(a, b, d);
    }

    let mut max_values = vec![i64::MIN; n];
    let mut heap = BinaryHeap::new();

    for (i, &cost) in costs.iter().enumerate() {
        max_values[i] = cost;
        heap.push(Node { cost, index: i });
    }

    while let Some(Node { cost, index }) = heap.pop() {
        for edge in &graph.edges[index] {
            let new_cost = cost + edge.weight as i64;
            if new_cost > max_values[edge.to] {
                max_values[edge.to] = new_cost;
                heap.push(Node { cost: new_cost, index: edge.to });
            }
        }
    }

    for i in 0..k {
        if i < max_values.len() {
            println!("{}", max_values[i]);
        }
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
    let (n, m, k) = (params[0], params[1], params[2]);

    let costs: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut edges = Vec::new();
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let edge_params: Vec<usize> = edge_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push((edge_params[0], edge_params[1], edge_params[2] as i32));
    }

    modified_shortest_path(n, m, k, costs, edges);
}
