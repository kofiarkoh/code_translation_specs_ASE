use std::collections::VecDeque;

const MODULO: i64 = 1_000_000_007; // Define the modulo constant

fn compute_tree_values(n: usize, a: &[usize], b: &[usize]) -> Vec<i64> {
    // Initialize the tans and tS arrays
    let mut tans = vec![1; n]; // Each node's value initialized to 1
    let mut tS = vec![0; n]; // Count of child nodes for each node

    // Build the adjacency list for the tree
    let mut tree = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        tree[a[i] - 1].push(b[i] - 1); // Adjust for 0-based indexing
        tree[b[i] - 1].push(a[i] - 1); // Adjust for 0-based indexing
    }

    // Perform a DFS to compute the values
    let mut visited = vec![false; n];
    let mut stack = vec![0]; // Start from the root node (assumed to be node 0)
    let mut post_order = Vec::new();

    while let Some(node) = stack.pop() {
        if visited[node] {
            post_order.push(node); // Add to post-order if already visited
        } else {
            visited[node] = true; // Mark as visited
            stack.push(node); // Push again to process after children
            for &child in &tree[node] {
                if !visited[child] {
                    stack.push(child); // Push unvisited children
                }
            }
        }
    }

    // Compute the values in reverse post-order
    for &node in post_order.iter().rev() {
        for &child in &tree[node] {
            if visited[child] { // Only process visited children
                tans[node] = (tans[node] * tans[child]) % MODULO; // Update parent's value
                tS[node] += 1; // Increment child count for the parent node
            }
        }
        // If the node has no children, its value remains 1 (initialized)
    }

    tans
}

fn main() {
    // Example usage
    let n = 5;
    let a = vec![1, 1, 3, 3]; // Adjusted for 1-based indexing
    let b = vec![2, 3, 4, 5]; // Adjusted for 1-based indexing
    let result = compute_tree_values(n, &a, &b);
    println!("{:?}", result); // Output the computed values
}
