use std::cmp::Ordering;
use std::collections::HashMap;
use std::usize;

const MAX_DISTANCE: usize = usize::MAX; // Represents an initialized maximum value for distance

fn compute_min_distance(num_nodes: usize, num_edges: usize, num_locations: usize, locations: Vec<usize>, edges: Vec<(usize, usize, usize)>) -> usize {
    // Preconditions check
    if num_nodes == 0 || num_edges == 0 || num_locations == 0 || locations.is_empty() || edges.is_empty() {
        return MAX_DISTANCE; // Undefined behavior for invalid inputs
    }

    // Initialize distance matrix
    let mut distance = vec![vec![MAX_DISTANCE; num_nodes]; num_nodes];
    
    // Set distance to self as 0
    for i in 0..num_nodes {
        distance[i][i] = 0;
    }

    // Process edges to fill the distance matrix
    for (u, v, weight) in edges {
        if u < num_nodes && v < num_nodes && weight >= 0 {
            distance[u][v] = distance[u][v].min(weight);
            distance[v][u] = distance[v][u].min(weight); // Assuming undirected graph
        }
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if distance[i][k] != MAX_DISTANCE && distance[k][j] != MAX_DISTANCE {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
    }

    // Generate all permutations of the locations to visit
    let mut min_distance = MAX_DISTANCE;
    let mut visited = vec![false; num_locations];
    let mut current_path = Vec::new();

    fn visit(
        locations: &Vec<usize>,
        distance: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        current_path: &mut Vec<usize>,
        current_distance: usize,
        min_distance: &mut usize,
    ) {
        if current_path.len() == locations.len() {
            // Check if we can return to the starting point
            let start = current_path[0];
            let last = current_path[current_path.len() - 1];
            let total_distance = current_distance + distance[last][start];
            *min_distance = (*min_distance).min(total_distance);
            return;
        }

        for i in 0..locations.len() {
            if !visited[i] {
                visited[i] = true;
                current_path.push(locations[i]);
                let next_distance = current_distance + if current_path.len() > 1 {
                    distance[current_path[current_path.len() - 2]][locations[i]]
                } else {
                    0 // No distance for the first node
                };
                visit(locations, distance, visited, current_path, next_distance, min_distance);
                current_path.pop();
                visited[i] = false;
            }
        }
    }

    visit(&locations, &distance, &mut visited, &mut current_path, 0, &mut min_distance);

    min_distance
}

fn main() {
    // Example usage
    let num_nodes = 4;
    let num_edges = 5;
    let num_locations = 3;
    let locations = vec![0, 1, 2];
    let edges = vec![(0, 1, 1), (1, 2, 2), (2, 3, 1), (0, 3, 4), (1, 3, 3)];

    let result = compute_min_distance(num_nodes, num_edges, num_locations, locations, edges);
    println!("Minimum distance: {}", result);
}
