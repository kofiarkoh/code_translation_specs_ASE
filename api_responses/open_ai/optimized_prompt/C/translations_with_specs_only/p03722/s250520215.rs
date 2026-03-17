use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
struct Edge {
    source: usize,
    destination: usize,
    cost: i32,
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(nodes: usize, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }

    fn min_cost(&self, start: usize, target: usize) -> Result<String, &'static str> {
        if self.nodes == 0 || self.edges.len() == 0 {
            return Err("Graph must have at least one node and one edge.");
        }

        // Initialize distances with a large value
        let mut dist = vec![i32::MAX; self.nodes];
        dist[start] = 0;

        // Relax edges up to (nodes - 1) times
        for _ in 0..self.nodes - 1 {
            for edge in &self.edges {
                if dist[edge.source] != i32::MAX {
                    let new_cost = dist[edge.source] + edge.cost;
                    if new_cost < dist[edge.destination] {
                        dist[edge.destination] = new_cost;
                    }
                }
            }
        }

        // Check for negative cycles
        let mut in_negative_cycle = vec![false; self.nodes];
        for _ in 0..self.nodes - 1 {
            for edge in &self.edges {
                if dist[edge.source] != i32::MAX {
                    let new_cost = dist[edge.source] + edge.cost;
                    if new_cost < dist[edge.destination] {
                        dist[edge.destination] = new_cost;
                        in_negative_cycle[edge.destination] = true;
                    }
                }
            }
        }

        // Propagate negative cycle influence
        for _ in 0..self.nodes {
            for edge in &self.edges {
                if in_negative_cycle[edge.source] {
                    in_negative_cycle[edge.destination] = true;
                }
            }
        }

        // If the target node is affected by a negative cycle
        if in_negative_cycle[target] {
            return Ok("inf".to_string());
        }

        // Return the negative of the minimum cost if no negative cycle affects the target
        if dist[target] == i32::MAX {
            return Ok("inf".to_string());
        }

        Ok((-dist[target]).to_string())
    }
}

fn main() {
    // Example usage
    let edges = vec![
        Edge { source: 0, destination: 1, cost: 1 },
        Edge { source: 1, destination: 2, cost: -1 },
        Edge { source: 2, destination: 0, cost: -1 },
    ];
    
    let graph = Graph::new(3, edges);
    match graph.min_cost(0, 2) {
        Ok(result) => println!("Minimum cost: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
