// Import necessary libraries for handling collections and numerical operations
use std::collections::VecDeque;

// Define a structure to represent an edge in the flow network
#[derive(Debug, Clone)] // Added Clone trait
struct Edge {
    to: usize,        // Destination node
    capacity: i32,    // Capacity of the edge
    cost: i32,        // Cost associated with the edge
    flow: i32,        // Current flow through the edge
}

// Define a structure to represent the flow network
struct FlowNetwork {
    nodes: usize,     // Number of nodes
    edges: Vec<Vec<Edge>>, // Adjacency list of edges
}

impl FlowNetwork {
    // Create a new flow network with a specified number of nodes
    fn new(nodes: usize) -> Self {
        FlowNetwork {
            nodes,
            edges: vec![Vec::new(); nodes],
        }
    }

    // Add an edge to the flow network
    fn add_edge(&mut self, from: usize, to: usize, capacity: i32, cost: i32) {
        self.edges[from].push(Edge {
            to,
            capacity,
            cost,
            flow: 0,
        });
    }

    // Function to calculate the minimum cost flow
    fn min_cost_flow(&mut self, source: usize, sink: usize, flow: i32) -> i32 {
        let mut total_cost = 0;
        let mut total_flow = 0;

        // While there is still flow to send
        while total_flow < flow {
            // Use a queue for BFS
            let mut queue = VecDeque::new();
            let mut dist = vec![i32::MAX; self.nodes];
            let mut parent = vec![None; self.nodes];

            // Start from the source
            queue.push_back(source);
            dist[source] = 0;

            // BFS to find the shortest path
            while let Some(node) = queue.pop_front() {
                for edge in &self.edges[node] {
                    // Check if we can push more flow through this edge
                    if edge.capacity - edge.flow > 0 && dist[edge.to] == i32::MAX {
                        dist[edge.to] = dist[node] + edge.cost;
                        parent[edge.to] = Some(node);
                        queue.push_back(edge.to);
                    }
                }
            }

            // If we didn't reach the sink, we can't send more flow
            if dist[sink] == i32::MAX {
                return -1; // Flow cannot be achieved
            }

            // Find the maximum flow we can send along the path found
            let mut path_flow = i32::MAX;
            let mut current = sink;

            while let Some(node) = parent[current] {
                for edge in &self.edges[node] {
                    if edge.to == current {
                        path_flow = path_flow.min(edge.capacity - edge.flow);
                    }
                }
                current = node;
            }

            // Update the flow and costs along the path
            current = sink;
            while let Some(node) = parent[current] {
                for edge in &mut self.edges[node] {
                    if edge.to == current {
                        edge.flow += path_flow; // Update flow
                    }
                }
                for edge in &mut self.edges[current] {
                    if edge.to == node {
                        edge.flow -= path_flow; // Update reverse flow
                    }
                }
                current = node;
            }

            total_flow += path_flow;
            total_cost += path_flow * dist[sink]; // Update total cost
        }

        total_cost // Return the total cost of the flow
    }
}

// Main function to demonstrate the flow network
fn main() {
    let n = 5; // Number of nodes
    let _m = 6; // Number of edges, prefixed with underscore to silence warning
    let mut network = FlowNetwork::new(n);

    // Add edges (from, to, capacity, cost)
    network.add_edge(0, 1, 10, 2);
    network.add_edge(1, 2, 5, 1);
    network.add_edge(0, 2, 15, 3);
    network.add_edge(2, 3, 10, 2);
    network.add_edge(1, 3, 10, 1);
    network.add_edge(3, 4, 10, 1);

    let flow = 10; // Desired flow
    let min_cost = network.min_cost_flow(0, 4, flow); // Calculate minimum cost from source (0) to sink (4)

    println!("Minimum cost to achieve the flow: {}", min_cost);
}
