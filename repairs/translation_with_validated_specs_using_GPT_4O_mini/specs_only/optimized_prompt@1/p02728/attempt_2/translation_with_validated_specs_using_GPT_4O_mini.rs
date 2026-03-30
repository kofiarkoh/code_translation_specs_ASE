use std::collections::VecDeque;

const MODULO: i64 = 1_000_000_007; // Define the modulo constant

fn compute_tree_values(n: usize, a: &[usize], b: &[usize]) -> Vec<i64> {
    // Initialize the tans and tS arrays
    let mut tans = vec![1; n]; // Each node's value initialized to 1
    let mut tS = vec![0; n]; // Count of child nodes for each node

    // Build the adjacency list for the tree
    let mut tree = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        tree[a[i]].push(b[i]);
        tree[b[i]].push(a[i]);
    }

    // Perform a BFS to compute the values
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(0); // Start from the root node (assumed to be node 0)
    visited[0] = true;

    while let Some(node) = queue.pop_front() {
        for &child in &tree[node] {
            if !visited[child] {
                visited[child] = true;
                queue.push_back(child);
                tS[node] += 1; // Increment child count for the parent node
            }
        }
    }

    // Post-order traversal to compute the values
    let mut stack = vec![0];
    let mut post_order = Vec::new();

    while let Some(node) = stack.pop() {
        post_order.push(node);
        for &child in &tree[node] {
            if visited[child] {
                visited[child] = false; // Mark as processed
                stack.push(child);
            }
        }
    }

    // Compute the values in reverse post-order
    for &node in post_order.iter().rev() {
        for &child in &tree[node] {
            if !visited[child] { // Only process unvisited children
                tans[node] = (tans[node] * tans[child]) % MODULO; // Update parent's value
            }
        }
    }

    tans
}

fn main() {
    // Example usage
    let n = 5;
    let a = vec![0, 0, 1, 1];
    let b = vec![1, 2, 3, 4];
    let result = compute_tree_values(n, &a, &b);
    println!("{:?}", result); // Output the computed values
}
