// Import necessary libraries
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // Read number of vertices and edges
    let (num_vertices, num_edges) = read_input();

    // Initialize the distance matrix
    let mut dist = vec![vec![i32::MAX; num_vertices]; num_vertices];
    for i in 0..num_vertices {
        dist[i][i] = 0; // Distance from a vertex to itself is zero
    }

    // Read edges and populate the distance matrix
    let edges = read_edges(num_edges);
    for (u, v, weight) in edges {
        dist[u][v] = weight; // Set the distance for the edge
        dist[v][u] = weight; // Assuming undirected graph
    }

    // Apply Floyd-Warshall algorithm to find shortest paths
    for k in 0..num_vertices {
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Count edges that are not part of any shortest path
    let mut non_shortest_path_edges = 0;
    for (u, v, weight) in edges {
        let is_part_of_shortest_path = (0..num_vertices).any(|i| {
            dist[i][u] + weight + dist[v][i] == dist[i][i] || 
            dist[i][v] + weight + dist[u][i] == dist[i][i]
        });
        if !is_part_of_shortest_path {
            non_shortest_path_edges += 1;
        }
    }

    // Output the count of edges not part of any shortest path
    println!("{}", non_shortest_path_edges);
}

// Function to read input values for vertices and edges
fn read_input() -> (usize, usize) {
    // Placeholder for reading input; replace with actual input handling
    (5, 7) // Example: 5 vertices and 7 edges
}

// Function to read edges from input
fn read_edges(num_edges: usize) -> Vec<(usize, usize, i32)> {
    // Placeholder for reading edges; replace with actual input handling
    vec![
        (0, 1, 10),
        (0, 2, 3),
        (1, 2, 1),
        (1, 3, 2),
        (2, 1, 4),
        (2, 3, 8),
        (3, 4, 7),
    ] // Example edges
}
