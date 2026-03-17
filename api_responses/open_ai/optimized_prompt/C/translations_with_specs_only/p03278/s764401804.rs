// Define a constant for the modulo operation
const MODULO: i64 = 1_000_000_007;

// Function to compute the combinatorial value based on the tree structure
fn compute_combinatorial_value(n: usize, edges: Vec<(usize, usize)>) -> i64 {
    // Ensure the number of nodes is positive
    if n == 0 {
        return 0; // This case should not happen as per the precondition
    }

    // Create an adjacency list to represent the tree
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edges {
        tree[u].push(v);
        tree[v].push(u);
    }

    // Dynamic programming table to store computed values
    let mut dp = vec![0; n];

    // Depth-first search to compute values
    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, dp: &mut Vec<i64>) {
        dp[node] = 1; // Initialize the dp value for the current node
        for &child in &tree[node] {
            if child != parent { // Avoid going back to the parent
                dfs(child, node, tree, dp);
                dp[node] = (dp[node] * (dp[child] + 1)) % MODULO; // Update dp value
            }
        }
    }

    // Start DFS from the root node (assumed to be node 0)
    dfs(0, usize::MAX, &tree, &mut dp);

    // The result is stored in dp[0], which is the value for the root node
    dp[0] // Return the computed combinatorial value
}

// Example usage (this part is not part of the specification but shows how to call the function)
fn main() {
    let n = 5; // Number of nodes
    let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)]; // Edges defining the tree
    let result = compute_combinatorial_value(n, edges);
    println!("{}", result); // Output the result
}
