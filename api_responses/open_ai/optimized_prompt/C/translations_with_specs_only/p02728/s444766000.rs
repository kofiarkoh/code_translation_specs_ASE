// Define a constant for the modulus as per the specification
const MODULUS: i64 = 1_000_000_007;

// Structure to represent a tree node
struct TreeNode {
    value: i64,
    children: Vec<usize>, // Indices of child nodes
}

// Function to compute values for each node in the tree
fn compute_tree_values(num_nodes: usize, edges: Vec<(usize, usize)>) -> Vec<i64> {
    // Create a vector of TreeNodes
    let mut tree: Vec<TreeNode> = vec![TreeNode { value: 0, children: Vec::new() }; num_nodes];

    // Build the tree structure from edges
    for (parent, child) in edges {
        tree[parent].children.push(child);
    }

    // Vector to hold the computed values
    let mut results = vec![0; num_nodes];

    // Depth-first search to compute values
    fn dfs(node_index: usize, tree: &mut Vec<TreeNode>, results: &mut Vec<i64>) {
        // Base value for the node can be set to its index or any other logic as per requirement
        results[node_index] = node_index as i64; // Initial value based on node index

        // Process each child node
        for &child in &tree[node_index].children {
            dfs(child, tree, results);
            // Update the current node's value based on the child's computed value
            results[node_index] = (results[node_index] + results[child]) % MODULUS;
        }
    }

    // Start DFS from the root node (assuming node 0 is the root)
    dfs(0, &mut tree, &mut results);

    // Return the computed values reduced by the modulus
    results.iter().map(|&val| val % MODULUS).collect()
}

// Example usage (this part is not part of the specification but shows how to call the function)
// fn main() {
//     let num_nodes = 5;
//     let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)];
//     let values = compute_tree_values(num_nodes, edges);
//     println!("{:?}", values);
// }
