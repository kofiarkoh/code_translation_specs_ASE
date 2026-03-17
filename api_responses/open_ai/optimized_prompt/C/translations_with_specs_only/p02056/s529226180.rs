use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    target: usize,
    cost: i32,
}

#[derive(Debug)]
struct Graph {
    vertices: usize,
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        self.edges.entry(from).or_insert_with(Vec::new).push(Edge { target: to, cost });
    }
}

#[derive(Debug)]
struct Node {
    vertex: usize,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Max-heap based on cost
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

fn modified_shortest_path(vertices: usize, edges: Vec<(usize, usize, i32)>, costs: Vec<i32>, k: usize) -> Vec<i32> {
    let mut graph = Graph::new(vertices);
    
    // Add edges to the graph
    for (from, to, cost) in edges {
        graph.add_edge(from, to, cost);
    }

    let mut max_values = vec![i32::MIN; k]; // Initialize max values
    let mut heap = BinaryHeap::new();
    let mut distances = vec![i32::MAX; vertices]; // Initialize distances

    // Start from vertex 0 (assuming 0-indexed)
    distances[0] = 0;
    heap.push(Node { vertex: 0, cost: 0 });

    while let Some(Node { vertex, cost }) = heap.pop() {
        if cost > distances[vertex] {
            continue; // Skip if we found a better path
        }

        for edge in graph.edges.get(&vertex).unwrap_or(&vec![]) {
            let next_cost = cost + edge.cost;
            if next_cost < distances[edge.target] {
                distances[edge.target] = next_cost;
                heap.push(Node { vertex: edge.target, cost: next_cost });
            }
        }
    }

    // Compute maximum values based on distances and costs
    for (i, &cost) in costs.iter().enumerate() {
        if i < k {
            max_values[i] = distances[i].max(cost);
        }
    }

    max_values
}

fn main() {
    // Example usage
    let vertices = 5;
    let edges = vec![(0, 1, 10), (0, 2, 5), (1, 2, 2), (1, 3, 1), (2, 1, 3), (2, 3, 9), (2, 4, 2), (3, 4, 4)];
    let costs = vec![1, 2, 3, 4, 5];
    let k = 3;

    let result = modified_shortest_path(vertices, edges, costs, k);
    println!("{:?}", result);
}
