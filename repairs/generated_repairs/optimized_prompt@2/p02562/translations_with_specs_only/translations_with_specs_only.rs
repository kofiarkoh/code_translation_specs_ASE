// Import necessary libraries
use std::collections::VecDeque;

struct FlowNetwork {
    num_nodes: usize,
    capacity: i64,
    costs: Vec<Vec<i64>>,
    flow: Vec<Vec<i64>>,
}

impl FlowNetwork {
    // Constructor to create a new FlowNetwork
    fn new(num_nodes: usize, capacity: i64, costs: Vec<Vec<i64>>) -> Self {
        // Initialize flow matrix with zeros
        let flow = vec![vec![0; num_nodes]; num_nodes];
        FlowNetwork {
            num_nodes,
            capacity,
            costs,
            flow,
        }
    }

    // Function to compute the maximum flow using a successive shortest path algorithm
    fn compute_max_flow(&mut self) -> Result<i64, &'static str> {
        // Check preconditions
        if self.num_nodes == 0 || self.capacity <= 0 {
            return Err("Invalid number of nodes or capacity");
        }
        if self.costs.len() != self.num_nodes || self.costs.iter().any(|row| row.len() != self.num_nodes) {
            return Err("Invalid cost matrix dimensions");
        }

        let source = 0; // Assuming source is node 0
        let sink = self.num_nodes - 1; // Assuming sink is the last node

        let mut max_flow = 0;

        // Main loop for finding augmenting paths
        while let Some(path_flow) = self.bfs(source, sink)? {
            max_flow += path_flow;
            self.update_flow(source, sink, path_flow);
        }

        Ok(max_flow)
    }

    // BFS to find the shortest augmenting path
    fn bfs(&self, source: usize, sink: usize) -> Result<Option<i64>, &'static str> {
        let mut visited = vec![false; self.num_nodes];
        let mut queue = VecDeque::new();
        let mut parent = vec![-1; self.num_nodes];
        let mut path_flow = i64::MAX;

        queue.push_back(source);
        visited[source] = true;

        while let Some(node) = queue.pop_front() {
            for next in 0..self.num_nodes {
                // Check for capacity and if not visited
                if !visited[next] && self.costs[node][next] - self.flow[node][next] > 0 {
                    visited[next] = true;
                    parent[next] = node as i64;
                    path_flow = path_flow.min(self.costs[node][next] - self.flow[node][next]);

                    if next == sink {
                        // Update parent for the path found
                        let mut current = next;
                        while current != source as usize {
                            let prev = parent[current] as usize;
                            parent[current] = prev as i64; // Update parent for backtracking
                            current = prev;
                        }
                        return Ok(Some(path_flow));
                    }
                    queue.push_back(next);
                }
            }
        }

        Ok(None) // No more augmenting paths
    }

    // Update flow values along the path found
    fn update_flow(&mut self, source: usize, sink: usize, path_flow: i64) {
        let mut current = sink;
        let mut parent = vec![-1; self.num_nodes]; // Initialize parent here
        while current != source {
            let prev = parent[current] as usize;
            self.flow[prev][current] += path_flow;
            self.flow[current][prev] -= path_flow; // For residual graph
            current = prev;
        }
    }

    // Function to visualize the flow configuration
    fn visualize_flow(&self) {
        for i in 0..self.num_nodes {
            for j in 0..self.num_nodes {
                if self.flow[i][j] > 0 {
                    println!("Flow from node {} to node {}: {}", i, j, self.flow[i][j]);
                }
            }
        }
    }
}

// Main function to demonstrate usage
fn main() {
    // Example usage
    let num_nodes = 4;
    let capacity = 10;
    let costs = vec![
        vec![0, 10, 5, 0],
        vec![0, 0, 15, 10],
        vec![0, 0, 0, 10],
        vec![0, 0, 0, 0],
    ];

    let mut flow_network = FlowNetwork::new(num_nodes, capacity, costs);
    
    match flow_network.compute_max_flow() {
        Ok(max_flow) => {
            println!("Maximum flow: {}", max_flow);
            flow_network.visualize_flow();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
