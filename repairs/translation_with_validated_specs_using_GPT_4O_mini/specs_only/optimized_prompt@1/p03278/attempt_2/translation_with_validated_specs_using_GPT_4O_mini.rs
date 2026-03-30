const MOD: i64 = 1_000_000_007;

fn compute_combinatorial_value(n: usize, edges: &[(usize, usize)]) -> i64 {
    // Step 1: Build the adjacency list for the tree
    let mut tree = vec![Vec::new(); n];
    for &(u, v) in edges {
        tree[u].push(v);
        tree[v].push(u);
    }

    // Step 2: Prepare dynamic programming tables
    let mut dp = vec![0; n]; // dp[i] will hold the count of configurations for subtree rooted at i
    let mut subtree_size = vec![0; n]; // subtree_size[i] will hold the size of the subtree rooted at i

    // Step 3: Depth-first search to compute subtree sizes and dp values
    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, dp: &mut Vec<i64>, subtree_size: &mut Vec<usize>) {
        subtree_size[node] = 1; // Count the current node
        dp[node] = 1; // Base case: one configuration for the node itself

        for &child in &tree[node] {
            if child != parent { // Avoid going back to the parent
                dfs(child, node, tree, dp, subtree_size);
                subtree_size[node] += subtree_size[child]; // Update the size of the current subtree
                dp[node] = (dp[node] * (dp[child] + 1)) % MOD; // Update configurations considering inclusion/exclusion
            }
        }
    }

    // Start DFS from the root node (0)
    dfs(0, usize::MAX, &tree, &mut dp, &mut subtree_size);

    // Step 4: Compute the final result based on dp values
    let mut result = 0;
    for i in 0..n {
        result = (result + dp[i]) % MOD; // Aggregate configurations from all nodes
    }

    result // Return the final computed result
}

fn main() {
    let n = 3; // Example number of nodes
    let edges = vec![(0, 1), (0, 2)]; // Example edges
    let result = compute_combinatorial_value(n, &edges);
    println!("{}", result); // Output the result
}
