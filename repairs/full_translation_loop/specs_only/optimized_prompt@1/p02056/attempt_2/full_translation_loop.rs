use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    target: usize,
    weight: i32,
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.edges.entry(from).or_insert_with(Vec::new).push(Edge { target: to, weight });
    }
}

#[derive(Debug)]
struct Node {
    vertex: usize,
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Max-heap based on distance
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Node {}

fn modified_shortest_path(num_vertices: usize, edges: Vec<(usize, usize, i32)>, costs: Vec<i32>, k: usize) -> Vec<i32> {
    let mut graph = Graph::new();
    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }

    let mut distances = vec![i32::MAX; num_vertices];
    distances[0] = 0; // Starting from vertex 0

    let mut heap = BinaryHeap::new();
    heap.push(Node { vertex: 0, distance: 0 });

    while let Some(Node { vertex, distance }) = heap.pop() {
        if distance > distances[vertex] {
            continue; // Skip if we found a better path
        }

        if let Some(neighbors) = graph.edges.get(&vertex) {
            for edge in neighbors {
                let new_distance = distance + edge.weight;
                if new_distance < distances[edge.target] {
                    distances[edge.target] = new_distance;
                    heap.push(Node { vertex: edge.target, distance: new_distance });
                }
            }
        }
    }

    // Compute maximum values based on distances and costs
    let mut results: Vec<i32> = distances.iter()
        .enumerate()
        .filter_map(|(i, &dist)| {
            if dist != i32::MAX {
                Some(dist + costs[i]) // Combine distances with costs
            } else {
                None
            }
        })
        .collect();

    results.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    results.truncate(k); // Keep only the top k values

    results
}

fn main() {
    // Example usage
    let num_vertices = 3;
    let edges = vec![(1, 2, 1), (5, 10, 15), (1, 2, 5), (2, 3, 10), (1, 3, 20)];
    let costs = vec![10, 20, 30];
    let k = 2;

    let max_values = modified_shortest_path(num_vertices, edges, costs, k);
    println!("{:?}", max_values);
}
