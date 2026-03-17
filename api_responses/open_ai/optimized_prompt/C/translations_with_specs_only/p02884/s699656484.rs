use std::collections::VecDeque;
use std::f64;

fn minimum_average_distance(num_nodes: usize, edges: Vec<(usize, usize)>, start_node: usize) -> f64 {
    // Step 1: Initialize distances
    let mut distances = vec![f64::INFINITY; num_nodes];
    distances[start_node] = 0.0;

    // Step 2: Create adjacency list for the directed graph
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
    for (from, to) in edges {
        graph[from].push(to);
    }

    // Step 3: Perform BFS to calculate minimum distances
    let mut queue = VecDeque::new();
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        for &neighbor in &graph[current] {
            // Update distance if a shorter path is found
            if distances[current] + 1.0 < distances[neighbor] {
                distances[neighbor] = distances[current] + 1.0;
                queue.push_back(neighbor);
            }
        }
    }

    // Step 4: Calculate the average distance
    let valid_distances: Vec<f64> = distances.iter().filter(|&&d| d < f64::INFINITY).collect();
    if valid_distances.is_empty() {
        // If no valid paths exist, return a large value
        return f64::INFINITY;
    }

    let total_distance: f64 = valid_distances.iter().sum();
    let average_distance = total_distance / valid_distances.len() as f64;

    // Step 5: Return the average distance formatted to ten decimal places
    format!("{:.10}", average_distance).parse().unwrap()
}

fn main() {
    // Example usage
    let num_nodes = 5;
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    let start_node = 0;

    let result = minimum_average_distance(num_nodes, edges, start_node);
    println!("{:.10}", result);
}
