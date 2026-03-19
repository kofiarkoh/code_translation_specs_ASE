use std::collections::VecDeque;

const MODULO: i64 = 1_000_000_007; // Predefined constant for modular arithmetic

// Structure to represent a tree node
#[derive(Clone)] // Derive Clone to allow creating multiple instances
struct TreeNode {
    value: i64,
    children: Vec<usize>, // Indices of child nodes
}

fn compute_tree_values(num_nodes: usize, edges: Vec<(usize, usize)>) -> Vec<i64> {
    // Create a tree structure from the edges
    let mut tree = vec![TreeNode { value: 0, children: Vec::new() }; num_nodes];

    // Build the tree from the edges
    for (parent, child) in edges {
        tree[parent].children.push(child);
    }

    // Result vector to hold computed values for each node
    let mut results = vec![0; num_nodes];

    // Perform a breadth-first traversal to compute values
    let mut queue = VecDeque::new();
    queue.push_back(0); // Start from the root node (assumed to be node 0)

    while let Some(node_index) = queue.pop_front() {
        let node = &tree[node_index];

        // Compute the value for the current node based on its children
        for &child_index in &node.children {
            queue.push_back(child_index);
            // Example transformation: accumulate child values
            results[node_index] = (results[node_index] + results[child_index]) % MODULO;
        }

        // Final adjustment for the current node (if needed)
        results[node_index] = (results[node_index] + 1) % MODULO; // Example adjustment
    }

    results
}

// Main function to demonstrate usage
fn main() {
    let num_nodes = 5;
    let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)];
    let results = compute_tree_values(num_nodes, edges);
    println!("{:?}", results);
}
