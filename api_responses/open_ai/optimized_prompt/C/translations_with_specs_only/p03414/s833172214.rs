// Directed graph edge classification based on connectivity criteria
fn classify_edges(num_nodes: usize, num_edges: usize, edges: Vec<(usize, usize)>) -> Vec<&'static str> {
    // Check for invalid input; behavior is undefined for invalid inputs
    if num_nodes == 0 || num_edges == 0 || edges.iter().any(|&(u, v)| u >= num_nodes || v >= num_nodes) {
        // Undefined behavior as per specification; no error handling implemented
        return vec![];
    }

    // Create a connectivity matrix to represent the directed graph
    let mut connectivity = vec![vec![false; num_nodes]; num_nodes];

    // Populate the connectivity matrix based on the edges
    for &(u, v) in &edges {
        connectivity[u][v] = true;
    }

    // Vector to hold the classification of each edge
    let mut classifications = Vec::with_capacity(num_edges);

    // Classify each edge based on connectivity
    for &(u, v) in &edges {
        // Check if there is a path from u to v and from v to u
        let is_diff = !connectivity[u][v] && !connectivity[v][u];
        classifications.push(if is_diff { "diff" } else { "same" });
    }

    classifications
}

// Example usage (commented out as per instruction to avoid additional functionality)
// fn main() {
//     let num_nodes = 3;
//     let num_edges = 3;
//     let edges = vec![(0, 1), (1, 2), (2, 0)];
//     let result = classify_edges(num_nodes, num_edges, edges);
//     for classification in result {
//         println!("{}", classification);
//     }
// }
