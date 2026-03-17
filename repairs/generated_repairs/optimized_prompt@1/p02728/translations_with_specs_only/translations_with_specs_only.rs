// Define a constant for the modular arithmetic
const MODULO: i64 = 1_000_000_007; // Example constant, can be adjusted as needed

// Function to compute values for each node in a tree based on edges
fn compute_tree_values(num_nodes: usize, edges: Vec<(usize, usize)>) -> Vec<i64> {
    // Initialize a vector to hold the computed values for each node
    let mut values = vec![0; num_nodes];

    // Create an adjacency list to represent the tree
    let mut tree = vec![Vec::new(); num_nodes];
    for (u, v) in edges {
        tree[u].push(v);
        tree[v].push(u);
    }

    // A simple DFS to compute values based on tree structure
    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, values: &mut Vec<i64>, depth: i64) {
        // Set the value based on the depth of the node
        values[node] = depth % MODULO;

        for &neighbor in &tree[node] {
            if neighbor != parent { // Avoid going back to the parent
                dfs(neighbor, node, tree, values, depth + 1);
            }
        }
    }

    // Start DFS from the root node (assumed to be node 0)
    dfs(0, usize::MAX, &tree, &mut values, 0);

    // Return the computed values for each node
    values
}

// Example usage
fn main() {
    let num_nodes = 5; // Example number of nodes
    let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)]; // Example edges forming a tree
    let results = compute_tree_values(num_nodes, edges);
    
    // Print the results
    for value in results {
        println!("{}", value);
    }
}
