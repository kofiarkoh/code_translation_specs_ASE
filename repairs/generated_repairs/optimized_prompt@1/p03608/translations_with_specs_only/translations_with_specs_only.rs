use std::cmp::Ordering;

const MAX_DISTANCE: i32 = i32::MAX; // Represents an unreachable distance

fn floyd_warshall(num_nodes: usize, edges: &[(usize, usize, i32)]) -> Vec<Vec<i32>> {
    let mut distance_matrix = vec![vec![MAX_DISTANCE; num_nodes]; num_nodes];

    // Initialize the distance matrix
    for i in 0..num_nodes {
        distance_matrix[i][i] = 0; // Distance to self is zero
    }

    // Populate the distance matrix with edge weights
    for &(u, v, weight) in edges {
        distance_matrix[u][v] = weight;
        distance_matrix[v][u] = weight; // Assuming undirected graph
    }

    // Floyd-Warshall algorithm to find shortest paths
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if distance_matrix[i][k] != MAX_DISTANCE && distance_matrix[k][j] != MAX_DISTANCE {
                    distance_matrix[i][j] = distance_matrix[i][j].min(distance_matrix[i][k] + distance_matrix[k][j]);
                }
            }
        }
    }

    distance_matrix
}

fn dfs(
    current: usize,
    visited: &mut Vec<bool>,
    locations: &[usize],
    distance_matrix: &Vec<Vec<i32>>,
    current_distance: i32,
    min_distance: &mut i32,
) {
    if visited.iter().all(|&v| v) {
        *min_distance = min_distance.min(current_distance);
        return;
    }

    for &next in locations {
        if !visited[next] {
            visited[next] = true;
            dfs(next, visited, locations, distance_matrix, current_distance + distance_matrix[current][next], min_distance);
            visited[next] = false; // Backtrack
        }
    }
}

fn minimum_distance(num_nodes: usize, num_edges: usize, num_locations: usize, locations: Vec<usize>, edges: Vec<(usize, usize, i32)>) -> i32 {
    if num_nodes == 0 || num_edges == 0 || num_locations == 0 || locations.is_empty() {
        return MAX_DISTANCE; // Invalid input case
    }

    let distance_matrix = floyd_warshall(num_nodes, &edges);
    let mut min_distance = MAX_DISTANCE;

    // Start DFS from the first location
    let mut visited = vec![false; num_nodes];
    for &start in &locations {
        visited[start] = true;
        dfs(start, &mut visited, &locations, &distance_matrix, 0, &mut min_distance);
        visited[start] = false; // Reset for next start
    }

    min_distance
}

fn main() {
    // Example usage
    let num_nodes = 5;
    let num_edges = 7;
    let num_locations = 3;
    let locations = vec![0, 1, 2];
    let edges = vec![
        (0, 1, 10),
        (0, 2, 15),
        (1, 2, 35),
        (1, 3, 20),
        (2, 3, 30),
        (3, 4, 25),
        (2, 4, 40),
    ];

    let result = minimum_distance(num_nodes, num_edges, num_locations, locations, edges);
    println!("Minimum distance: {}", result);
}
