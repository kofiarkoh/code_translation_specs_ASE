use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();
    
    // Read number of vertices
    let num_vertices: usize = parts.next().unwrap().parse().expect("Invalid input for number of vertices");
    // Read number of edges
    let num_edges: usize = parts.next().unwrap().parse().expect("Invalid input for number of edges");

    // Handle the case where the number of vertices is zero
    if num_vertices == 0 {
        return;
    }

    // Initialize the cost matrix with "infinity"
    const INF: i32 = i32::MAX;
    let mut cost_matrix = vec![vec![INF; num_vertices]; num_vertices];

    // Read edges and populate the cost matrix
    for _ in 0..num_edges {
        let u: usize = parts.next().unwrap().parse().expect("Invalid input for vertex u");
        let v: usize = parts.next().unwrap().parse().expect("Invalid input for vertex v");
        let weight: i32 = parts.next().unwrap().parse().expect("Invalid input for edge weight");

        // Ensure the vertices are within valid range
        if u < num_vertices && v < num_vertices {
            cost_matrix[u][v] = weight;
            cost_matrix[v][u] = weight; // Undirected graph
        }
    }

    // Prim's algorithm to find the total weight of the MST
    let mut total_weight = 0;
    let mut min_cost = vec![INF; num_vertices];
    let mut used = vec![false; num_vertices];
    min_cost[0] = 0; // Start from the first vertex

    for _ in 0..num_vertices {
        // Find the vertex with the minimum cost that is not yet included in the MST
        let mut min_index = usize::MAX;
        let mut min_value = INF;

        for v in 0..num_vertices {
            if !used[v] && min_cost[v] < min_value {
                min_value = min_cost[v];
                min_index = v;
            }
        }

        // If no vertex is found, the graph is not connected
        if min_index == usize::MAX {
            // Undefined behavior as per specification
            return;
        }

        // Include this vertex in the MST
        used[min_index] = true;
        total_weight += min_value;

        // Update the minimum costs for the adjacent vertices
        for v in 0..num_vertices {
            if cost_matrix[min_index][v] != INF && !used[v] && cost_matrix[min_index][v] < min_cost[v] {
                min_cost[v] = cost_matrix[min_index][v];
            }
        }
    }

    // Output the total weight of the MST
    println!("{}", total_weight);
}
