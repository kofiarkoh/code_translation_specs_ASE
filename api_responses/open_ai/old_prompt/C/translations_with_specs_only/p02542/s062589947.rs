use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    to: usize,
    capacity: i64,
    flow: i64,
    cost: i64,
}

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<usize>>, // adjacency list
    edges: Vec<Edge>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); n],
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i64, cost: i64) {
        let edge = Edge { to, capacity, flow: 0, cost };
        self.adj[from].push(self.edges.len());
        self.edges.push(edge);
    }
}

#[derive(Debug)]
struct State {
    cost: i64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap based on cost
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Eq for State {}

fn successive_shortest_path(graph: &mut Graph, source: usize, sink: usize, b: &[i64]) -> i64 {
    let n = graph.adj.len();
    let mut dist = vec![i64::MAX; n];
    let mut prev = vec![None; n];
    let mut in_queue = vec![false; n];
    let mut min_heap = BinaryHeap::new();

    dist[source] = 0;
    min_heap.push(State { cost: 0, node: source });

    while let Some(State { cost, node }) = min_heap.pop() {
        if cost > dist[node] {
            continue; // Skip if we have found a better path
        }

        for &edge_index in &graph.adj[node] {
            let edge = &graph.edges[edge_index];
            let residual_capacity = edge.capacity - edge.flow;

            if residual_capacity > 0 {
                let new_cost = cost + edge.cost;

                if new_cost < dist[edge.to] {
                    dist[edge.to] = new_cost;
                    prev[edge.to] = Some(edge_index);
                    min_heap.push(State { cost: new_cost, node: edge.to });
                }
            }
        }
    }

    // Flow adjustment and cost calculation
    let mut total_cost = 0;
    let mut current_node = sink;

    while let Some(edge_index) = prev[current_node] {
        let edge = &mut graph.edges[edge_index];
        let flow_to_add = b[current_node]; // Simplified for this example
        edge.flow += flow_to_add;
        total_cost += flow_to_add * edge.cost;
        current_node = edge.to; // Move to the previous node
    }

    -total_cost // Return negative of the minimum cost
}

fn main() {
    // Example usage
    let n = 5; // Number of nodes
    let mut graph = Graph::new(n);
    
    // Add edges (example values)
    graph.add_edge(0, 1, 10, 2);
    graph.add_edge(1, 2, 5, 1);
    graph.add_edge(0, 2, 15, 3);
    
    let b = vec![0, 10, -10, 0, 0]; // Supply/Demand at each node
    let min_cost = successive_shortest_path(&mut graph, 0, 2, &b);
    
    println!("{}", min_cost); // Output the negative of the minimum cost
}
