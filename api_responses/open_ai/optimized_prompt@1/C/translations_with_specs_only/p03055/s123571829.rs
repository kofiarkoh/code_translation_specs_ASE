use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // The number of edges must be 2n - 2
    let expected_edges = 2 * n - 2;
    let mut edges = Vec::new();

    // Read edges
    for _ in 0..expected_edges {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push((edge[0], edge[1]));
    }

    // Build the tree as an adjacency list
    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
    for (u, v) in edges {
        tree.entry(u).or_insert_with(Vec::new).push(v);
        tree.entry(v).or_insert_with(Vec::new).push(u);
    }

    // Function to calculate the maximum depth using DFS
    fn max_depth(node: usize, parent: usize, tree: &HashMap<usize, Vec<usize>>) -> usize {
        let mut depth = 0;
        for &neighbor in &tree[&node] {
            if neighbor != parent { // Avoid going back to the parent
                depth = depth.max(max_depth(neighbor, node, tree));
            }
        }
        depth + 1 // Count the current node
    }

    // Start DFS from an arbitrary root node, here we assume node 1 exists
    let depth = max_depth(1, 0, &tree);

    // Determine the winner based on the maximum depth
    let winner = if depth % 2 == 1 { "First" } else { "Second" };
    println!("{}", winner);
}
