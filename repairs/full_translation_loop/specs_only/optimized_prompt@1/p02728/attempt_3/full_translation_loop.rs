use std::collections::VecDeque;

const MODULO: i64 = 1_000_000_007; // Predefined constant for modular arithmetic

// Structure to represent a tree node
#[derive(Clone)] // Derive Clone to allow creating multiple instances
struct TreeNode {
    children: Vec<usize>, // Indices of child nodes
}

fn compute_tree_values(num_nodes: usize, edges: Vec<(usize, usize)>) -> Vec<i64> {
    // Create a tree structure from the edges
    let mut tree = vec![TreeNode { children: Vec::new() }; num_nodes];

    // Build the tree from the edges
    for (parent, child) in edges {
        tree[parent].children.push(child);
    }

    // Result vector to hold computed values for each node
    let mut results = vec![0; num_nodes];

    // Perform a depth-first traversal to compute values
    let mut stack = vec![0]; // Start from the root node (assumed to be node 0)
    let mut visited = vec![false; num_nodes];

    while let Some(node_index) = stack.pop() {
        if visited[node_index] {
            // If already visited, compute the value for the current node
            let mut total = 0;
            for &child_index in &tree[node_index].children {
                total = (total + results[child_index]) % MODULO;
            }
            results[node_index] = (total + 1) % MODULO; // Example adjustment
        } else {
            // Mark as visited and push back to stack for processing after children
            visited[node_index] = true;
            stack.push(node_index); // Push current node back to process after children
            for &child_index in &tree[node_index].children {
                stack.push(child_index); // Push children to stack
            }
        }
    }

    results
}

// Main function to demonstrate usage
fn main() {
    let num_nodes = 5;
    let edges = vec![(1, 2), (1, 3), (3, 4), (3, 5)]; // Adjusted edges to match input format
    let results = compute_tree_values(num_nodes, edges);
    for result in results {
        println!("{}", result);
    }
}
