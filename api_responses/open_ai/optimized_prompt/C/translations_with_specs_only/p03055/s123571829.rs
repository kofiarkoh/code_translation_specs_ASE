use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // The number of edges must be 2 * n - 2
    let expected_edges = 2 * n - 2;
    let mut edges = Vec::with_capacity(expected_edges);

    // Read edges
    for _ in 0..expected_edges {
        let edge: Vec<usize> = lines.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push((edge[0], edge[1]));
    }

    // Build the tree structure
    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
    for (u, v) in edges {
        tree.entry(u).or_insert_with(Vec::new).push(v);
        tree.entry(v).or_insert_with(Vec::new).push(u);
    }

    // Calculate the depth of the tree using DFS
    let depth = calculate_depth(&tree, 1, 0, &mut vec![false; n + 1]);

    // Determine the winner based on the depth
    let winner = if depth % 2 == 0 { "Second" } else { "First" };
    println!("{}", winner);
}

// Function to calculate the depth of the tree using DFS
fn calculate_depth(tree: &HashMap<usize, Vec<usize>>, node: usize, current_depth: usize, visited: &mut Vec<bool>) -> usize {
    visited[node] = true;
    let mut max_depth = current_depth;

    for &neighbor in &tree[&node] {
        if !visited[neighbor] {
            let depth = calculate_depth(tree, neighbor, current_depth + 1, visited);
            max_depth = max_depth.max(depth);
        }
    }

    max_depth
}
