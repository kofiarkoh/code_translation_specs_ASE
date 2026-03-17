// Import necessary libraries
use std::collections::{HashMap, HashSet};

// Function to compute the maximum cost of a path in a directed graph
fn max_path_cost(num_vertices: usize, sources: &[usize], destinations: &[usize], costs: &[usize]) -> usize {
    // Create a graph representation using an adjacency list
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    
    // Populate the graph with edges and their costs
    for i in 0..sources.len() {
        let src = sources[i];
        let dest = destinations[i];
        let cost = costs[i];
        
        // Add the edge to the graph
        graph.entry(src).or_insert_with(Vec::new).push((dest, cost));
    }

    // Initialize a vector to track the maximum cost to reach each vertex
    let mut max_costs = vec![0; num_vertices];
    // Set of visited vertices to prevent revisiting
    let mut visited = HashSet::new();

    // Depth-first search (DFS) function to explore the graph
    fn dfs(
        vertex: usize,
        graph: &HashMap<usize, Vec<(usize, usize)>>,
        max_costs: &mut Vec<usize>,
        visited: &mut HashSet<usize>,
        current_cost: usize,
    ) {
        // Update the maximum cost to reach this vertex
        max_costs[vertex] = max_costs[vertex].max(current_cost);

        // Explore neighbors
        if let Some(neighbors) = graph.get(&vertex) {
            for &(neighbor, cost) in neighbors {
                // Only visit unvisited neighbors
                if !visited.contains(&neighbor) {
                    // Mark the vertex as visited
                    visited.insert(neighbor);
                    // Recursively visit the neighbor with updated cost
                    dfs(neighbor, graph, max_costs, visited, current_cost + cost);
                    // Unmark the vertex after visiting to allow other paths
                    visited.remove(&neighbor);
                }
            }
        }
    }

    // Start DFS from the starting vertex (assumed to be vertex 0)
    visited.insert(0);
    dfs(0, &graph, &mut max_costs, &mut visited, 0);

    // Return the maximum cost found
    *max_costs.iter().max().unwrap_or(&0)
}

// Example usage (commented out as per the specification)
fn main() {
    // let num_vertices = 5;
    // let sources = vec![0, 1, 1, 0, 0];
    // let destinations = vec![1, 2, 4, 2, 3];
    // let costs = vec![1, 1, 4, 2, 2];
    // let result = max_path_cost(num_vertices, &sources, &destinations, &costs);
    // println!("Maximum path cost: {}", result);
}
