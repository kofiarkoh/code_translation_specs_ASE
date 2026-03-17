// Import necessary libraries
use std::cmp::min;
use std::collections::BinaryHeap;
use std::io::{self, Write};

// Define constants for maximum nodes and edges
const MAX_NODES: usize = 51;
const INF: i64 = i64::MAX; // Use i64 to represent infinity for costs

// Structure to represent an edge in the flow network
#[derive(Clone)]
struct Edge {
    to: usize,
    capacity: i64,
    cost: i64,
    flow: i64,
}

// Structure to represent the flow network
struct FlowNetwork {
    nodes: usize,
    edges: Vec<Vec<Edge>>,
    costs: [[i64; MAX_NODES]; MAX_NODES],
}

impl FlowNetwork {
    // Create a new flow network with a given number of nodes
    fn new(nodes: usize, costs: [[i64; MAX_NODES]; MAX_NODES]) -> Self {
        let mut edges = vec![Vec::new(); nodes];
        for i in 0..nodes {
            for j in 0..nodes {
                if costs[i][j] < 0 {
                    panic!("Cost matrix must not contain negative values");
                }
                if costs[i][j] > 0 {
                    edges[i].push(Edge {
                        to: j,
                        capacity: INF, // Assume infinite capacity for simplicity
                        cost: costs[i][j],
                        flow: 0,
                    });
                }
            }
        }
        FlowNetwork { nodes, edges, costs }
    }

    // Function to compute the successive shortest path
    fn successive_shortest_path(&mut self) -> i64 {
        let mut total_cost = 0;
        loop {
            let (dist, prev) = self.shortest_path();
            if dist[self.nodes - 1] == INF {
                break; // No more augmenting path
            }
            let mut flow = INF;
            let mut v = self.nodes - 1;
            while v != 0 {
                let u = prev[v];
                let edge = self.edges[u].iter_mut().find(|e| e.to == v).unwrap();
                flow = min(flow, edge.capacity - edge.flow);
                v = u;
            }
            v = self.nodes - 1;
            while v != 0 {
                let u = prev[v];
                let edge = self.edges[u].iter_mut().find(|e| e.to == v).unwrap();
                edge.flow += flow;
                total_cost += flow * edge.cost;
                v = u;
            }
        }
        total_cost
    }

    // Function to find the shortest path using Dijkstra's algorithm
    fn shortest_path(&self) -> (Vec<i64>, Vec<usize>) {
        let mut dist = vec![INF; self.nodes];
        let mut prev = vec![0; self.nodes];
        let mut heap = BinaryHeap::new();
        dist[0] = 0;
        heap.push((0, 0)); // (cost, node)

        while let Some((cost, u)) = heap.pop() {
            if -cost > dist[u] {
                continue; // Skip if we found a better path
            }
            for edge in &self.edges[u] {
                if edge.flow < edge.capacity {
                    let new_cost = dist[u] + edge.cost;
                    if new_cost < dist[edge.to] {
                        dist[edge.to] = new_cost;
                        prev[edge.to] = u;
                        heap.push((-new_cost, edge.to));
                    }
                }
            }
        }
        (dist, prev)
    }

    // Function to print the flow configuration
    fn print_flow(&self) {
        let mut flow_matrix = vec![vec!['.'; self.nodes]; self.nodes];
        for u in 0..self.nodes {
            for edge in &self.edges[u] {
                if edge.flow > 0 {
                    flow_matrix[u][edge.to] = 'X'; // Mark active flow
                }
            }
        }
        for row in flow_matrix {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

// Main function to read input and execute the flow network algorithm
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = params[0];
    let k = params[1];

    if n == 0 || k < 0 {
        panic!("Invalid input: N must be positive and K must be non-negative");
    }

    let mut costs = [[0; MAX_NODES]; MAX_NODES];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for j in 0..n {
            costs[i][j] = row[j];
        }
    }

    let mut network = FlowNetwork::new(n, costs);
    let total_cost = network.successive_shortest_path();
    println!("Total cost: {}", total_cost);
    network.print_flow();
}
