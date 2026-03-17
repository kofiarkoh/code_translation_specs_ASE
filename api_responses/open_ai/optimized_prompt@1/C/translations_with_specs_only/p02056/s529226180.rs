use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Debug)]
struct Graph {
    vertices: usize,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: vec![Vec::new(); vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.edges[from].push(Edge { to, weight });
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    vertex: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a max-heap, so we reverse the ordering
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn modified_shortest_path(graph: &Graph, costs: &[i32], k: usize) -> Result<Vec<i32>, Box<dyn Error>> {
    if graph.vertices == 0 || costs.len() != graph.vertices {
        return Err("Invalid graph or costs".into());
    }

    let mut distances = vec![i32::MAX; graph.vertices];
    let mut heap = BinaryHeap::new();

    // Initialize distances with vertex costs
    for (i, &cost) in costs.iter().enumerate() {
        distances[i] = cost;
        heap.push(State { cost, vertex: i });
    }

    while let Some(State { cost, vertex }) = heap.pop() {
        if cost > distances[vertex] {
            continue; // Skip if we found a better path
        }

        for edge in &graph.edges[vertex] {
            let next_cost = cost + edge.weight;
            if next_cost < distances[edge.to] {
                distances[edge.to] = next_cost;
                heap.push(State { cost: next_cost, vertex: edge.to });
            }
        }
    }

    // Collect the k maximum values from distances
    let mut max_values: Vec<i32> = distances.into_iter().collect();
    max_values.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    max_values.truncate(k); // Keep only the top k values

    Ok(max_values)
}

fn main() -> Result<(), Box<dyn Error>> {
    let vertices = 5; // Example input
    let edges = 6; // Example input
    let k = 3; // Example input
    let costs = vec![1, 2, 3, 4, 5]; // Example costs

    let mut graph = Graph::new(vertices);
    graph.add_edge(0, 1, 2);
    graph.add_edge(1, 2, 3);
    graph.add_edge(2, 3, 4);
    graph.add_edge(3, 4, 5);
    graph.add_edge(0, 4, 10);
    graph.add_edge(1, 3, 1);

    let max_values = modified_shortest_path(&graph, &costs, k)?;
    println!("{:?}", max_values);

    Ok(())
}
