use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the graph
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    
    // Read the graph structure
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let node_index: usize = parts.next().unwrap().parse().unwrap();
        let edge_count: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..edge_count {
            let dest_index: usize = parts.next().unwrap().parse().unwrap();
            let cost: usize = parts.next().unwrap().parse().unwrap();
            graph[node_index].push((dest_index, cost));
        }
    }

    // The first node specified is treated as the source node
    let source_node = 0;

    // Initialize costs
    let mut costs = vec![-1; n]; // -1 indicates unreachable
    costs[source_node] = 0; // Cost to source node is 0

    // Visited nodes
    let mut visited = vec![false; n];

    // Dijkstra's algorithm (O(n^2) implementation)
    for _ in 0..n {
        let mut min_cost = i32::MAX;
        let mut current_node = None;

        // Find the unvisited node with the smallest cost
        for node in 0..n {
            if !visited[node] && costs[node] >= 0 && costs[node] < min_cost {
                min_cost = costs[node];
                current_node = Some(node);
            }
        }

        // If no unvisited node is reachable, break
        let current_node = match current_node {
            Some(node) => node,
            None => break,
        };

        // Mark the current node as visited
        visited[current_node] = true;

        // Update costs for neighbors
        for &(neighbor, edge_cost) in &graph[current_node] {
            if !visited[neighbor] {
                let new_cost = costs[current_node] + edge_cost as i32;
                if costs[neighbor] == -1 || new_cost < costs[neighbor] {
                    costs[neighbor] = new_cost;
                }
            }
        }
    }

    // Output the results
    for (index, &cost) in costs.iter().enumerate() {
        println!("{} {}", index, cost);
    }
}
