use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap(),
        _ => return, // Handle case where input is invalid
    };
    
    // Initialize the adjacency list for the tree
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    
    // Read edges
    for _ in 0..(2 * n - 2) {
        let edge: Vec<usize> = match lines.next() {
            Some(Ok(line)) => line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            _ => return, // Handle case where input is invalid
        };
        edges.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        edges.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }
    
    // Function to calculate the maximum depth using DFS
    fn max_depth(node: usize, parent: usize, edges: &HashMap<usize, Vec<usize>>) -> usize {
        let mut depth = 0;
        for &neighbor in &edges[&node] {
            if neighbor != parent { // Avoid going back to the parent
                depth = depth.max(max_depth(neighbor, node, edges));
            }
        }
        depth + 1 // Add one for the current node
    }
    
    // Start DFS from an arbitrary root node (1 in this case)
    let depth = max_depth(1, 0, &edges);
    
    // Determine the winner based on the maximum depth
    if depth % 2 == 1 {
        println!("First");
    } else {
        println!("Second");
    }
}
