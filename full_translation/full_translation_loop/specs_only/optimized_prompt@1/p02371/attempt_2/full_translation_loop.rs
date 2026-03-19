use std::collections::HashMap;

struct Graph {
    edges: Vec<(usize, usize, usize)>, // (source, destination, cost)
    num_vertices: usize,
}

impl Graph {
    fn new(num_vertices: usize, sources: Vec<usize>, destinations: Vec<usize>, costs: Vec<usize>) -> Self {
        let edges = sources.into_iter()
            .zip(destinations.into_iter())
            .zip(costs.into_iter())
            .map(|((src, dst), cost)| (src, dst, cost))
            .collect();
        Graph { edges, num_vertices }
    }

    fn max_path_cost(&self, start_vertex: usize) -> usize {
        let mut max_costs = vec![0; self.num_vertices]; // Initialize costs to 0
        let mut visited = vec![false; self.num_vertices]; // Track visited vertices
        self.dfs(start_vertex, 0, &mut max_costs, &mut visited);
        *max_costs.iter().max().unwrap_or(&0) // Return the maximum cost found
    }

    fn dfs(&self, vertex: usize, current_cost: usize, max_costs: &mut Vec<usize>, visited: &mut Vec<bool>) {
        if visited[vertex] {
            return; // Prevent revisiting
        }
        visited[vertex] = true; // Mark the vertex as visited
        max_costs[vertex] = current_cost; // Update the cost to reach this vertex

        for &(src, dst, cost) in &self.edges {
            if src == vertex {
                // Explore the edge
                self.dfs(dst, current_cost + cost, max_costs, visited);
            }
        }
        visited[vertex] = false; // Reset visited state for the next search
    }
}

fn main() {
    // Example usage:
    let num_vertices = 5;
    let sources = vec![0, 0, 1, 1]; // Adjusted to match the input format
    let destinations = vec![1, 2, 3, 4];
    let costs = vec![1, 2, 1, 4];

    let graph = Graph::new(num_vertices, sources, destinations, costs);
    let max_cost = graph.max_path_cost(0); // Starting from vertex 0
    println!("{}", max_cost); // Output the maximum path cost
}
