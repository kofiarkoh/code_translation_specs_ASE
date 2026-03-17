use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    // Read the number of nodes from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of nodes (n)
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for number of nodes");

    // Edge case: If n is 1, the winner is "First"
    if n == 1 {
        println!("First");
        return;
    }

    // Initialize adjacency list for the tree
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Read edges and construct the tree
    for _ in 0..n - 1 {
        let edge = lines.next().unwrap().unwrap();
        let nodes: Vec<usize> = edge.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for edge"))
            .collect();
        edges[nodes[0]].push(nodes[1]);
        edges[nodes[1]].push(nodes[0]);
    }

    // Array to store the depth of each node
    let mut depth = vec![-1; n];
    depth[0] = 0; // Start from node 0

    // BFS to calculate the depth of each node
    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &edges[node] {
            if depth[neighbor] == -1 { // If not visited
                depth[neighbor] = depth[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    // Determine the maximum depth
    let max_depth = *depth.iter().max().unwrap();

    // Determine the winner based on the maximum depth
    if max_depth % 2 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
