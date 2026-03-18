use std::collections::HashMap;
use std::i32;

fn compute_min_cost(num_nodes: usize, num_edges: usize, edges: Vec<(usize, usize, i32)>, target_node: usize) -> String {
    // Initialize distances with "inf" (using a large value)
    let mut distances: Vec<i32> = vec![i32::MAX; num_nodes];
    distances[0] = 0; // Starting node cost is 0

    // Relax edges up to num_nodes - 1 times
    for _ in 0..num_nodes - 1 {
        for &(src, dest, cost) in &edges {
            if distances[src] != i32::MAX && distances[src] + cost < distances[dest] {
                distances[dest] = distances[src] + cost;
            }
        }
    }

    // Check for negative cycles affecting the target node
    for _ in 0..num_nodes - 1 {
        for &(src, dest, cost) in &edges {
            if distances[src] != i32::MAX && distances[src] + cost < distances[dest] {
                distances[dest] = i32::MIN; // Mark as affected by negative cycle
            }
        }
    }

    // If the target node is affected by a negative cycle, return "inf"
    if distances[target_node] == i32::MIN {
        return "inf".to_string();
    }

    // If a valid path exists, return the minimum cost (as a negative value)
    if distances[target_node] != i32::MAX {
        return (-distances[target_node]).to_string();
    }

    // If no valid path exists, return "inf"
    "inf".to_string()
}

// Example usage (commented out as per instructions):
// fn main() {
//     let num_nodes = 5;
//     let num_edges = 8;
//     let edges = vec![(0, 1, -1), (0, 2, 4), (1, 2, 3), (1, 3, 2), (1, 4, 2), (3, 1, 1), (4, 3, -3)];
//     let target_node = 1;
//     let result = compute_min_cost(num_nodes, num_edges, edges, target_node);
//     println!("{}", result); // Expected output: "inf" or a negative cost
// }
