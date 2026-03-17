use std::collections::VecDeque;

const MOD: i64 = 1_000_000_007;

fn main() {
    let n: usize = read_input(); // Assume this function reads the number of nodes
    let edges: Vec<(usize, usize)> = read_edges(n); // Assume this function reads edges

    // Preconditions: n must be at least 1 and edges must be n-1
    assert!(n >= 1);
    assert_eq!(edges.len(), n - 1);

    // Initialize DP and size arrays
    let mut dp = vec![vec![0; n]; n];
    let mut sz = vec![0; n];

    // Build adjacency list for the tree
    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    // Perform DFS to compute dp and sz
    dfs(0, usize::MAX, &graph, &mut dp, &mut sz);

    // The answer is stored in dp[0][0] after the DFS
    let ans = dp[0][0];
    println!("{}", ans);
}

// DFS function to compute sizes and DP values
fn dfs(node: usize, parent: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<i64>>, sz: &mut Vec<i64>) {
    sz[node] = 1; // Initialize size of the current node's subtree
    dp[node][0] = 1; // Base case: one way to pair zero nodes

    for &neighbor in &graph[node] {
        if neighbor == parent {
            continue; // Skip the parent node
        }
        dfs(neighbor, node, graph, dp, sz); // Recursive DFS

        // Update dp and sz for the current node based on the child
        for i in (0..sz[node] as usize + 1).rev() {
            for j in 1..=sz[neighbor] as usize {
                dp[node][i + j] = (dp[node][i + j] + dp[node][i] * dp[neighbor][j] % MOD) % MOD;
            }
        }
        sz[node] += sz[neighbor]; // Update the size of the current node's subtree
    }
}

// Placeholder function to read input for n
fn read_input() -> usize {
    // Implementation to read an integer from input
    5 // Example value
}

// Placeholder function to read edges
fn read_edges(n: usize) -> Vec<(usize, usize)> {
    // Implementation to read edges from input
    vec![(0, 1), (1, 2), (1, 3), (3, 4)] // Example edges for a tree
}
