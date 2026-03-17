use std::collections::HashMap;
use std::io::{self, Write};

struct Node {
    edges: Vec<(usize, u32)>, // (destination node, cost)
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Graph {
            nodes: vec![Node { edges: Vec::new() }; num_nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: u32) {
        self.nodes[from].edges.push((to, cost));
    }

    fn dijkstra(&self, source: usize) -> Vec<i32> {
        let num_nodes = self.nodes.len();
        let mut costs = vec![-1; num_nodes]; // Initialize costs to -1
        let mut visited = vec![false; num_nodes];
        let mut current_costs = vec![u32::MAX; num_nodes]; // Use u32::MAX to represent infinity
        current_costs[source] = 0; // Cost to source is 0
        costs[source] = 0; // Cost to source is 0

        for _ in 0..num_nodes {
            let mut min_cost = u32::MAX;
            let mut min_index = usize::MAX;

            // Find the unvisited node with the smallest cost
            for i in 0..num_nodes {
                if !visited[i] && current_costs[i] < min_cost {
                    min_cost = current_costs[i];
                    min_index = i;
                }
            }

            if min_index == usize::MAX {
                break; // All reachable nodes have been visited
            }

            visited[min_index] = true;

            // Update costs for neighbors
            for &(neighbor, edge_cost) in &self.nodes[min_index].edges {
                let new_cost = current_costs[min_index].saturating_add(edge_cost);
                if new_cost < current_costs[neighbor] {
                    current_costs[neighbor] = new_cost;
                    costs[neighbor] = new_cost as i32; // Update cost
                }
            }
        }

        costs
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_nodes: usize = input.trim().parse().expect("Invalid number of nodes");

    let mut graph = Graph::new(num_nodes);

    for i in 0..num_nodes {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edges: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();

        let num_edges = edges[0] as usize;
        for j in 0..num_edges {
            let dest = edges[2 * j + 1] as usize;
            let cost = edges[2 * j + 2];
            graph.add_edge(i, dest, cost);
        }
    }

    let source = 0; // Assuming the source node is always the first node
    let costs = graph.dijkstra(source);

    for (index, &cost) in costs.iter().enumerate() {
        println!("{} {}", index, cost);
    }
}
