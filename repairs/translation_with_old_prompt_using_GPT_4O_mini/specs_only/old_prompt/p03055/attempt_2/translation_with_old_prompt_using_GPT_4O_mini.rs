use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    // Read the number of nodes
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Edge case: if n is 1, the winner is "First"
    if n == 1 {
        println!("First");
        return;
    }

    // Initialize adjacency list for the tree
    let mut edges = vec![Vec::new(); n];
    
    // Read edges
    for _ in 0..n - 1 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        
        // Ensure indices are within bounds
        if u < n && v < n {
            edges[u].push(v);
            edges[v].push(u);
        }
    }

    // Depth array to store the depth of each node
    let mut depth = vec![-1; n];
    let mut queue = VecDeque::new();
    
    // Start BFS from node 0
    depth[0] = 0;
    queue.push_back(0);
    
    // Perform BFS to calculate depths
    while let Some(node) = queue.pop_front() {
        for &neighbor in &edges[node] {
            if depth[neighbor] == -1 { // Not visited
                depth[neighbor] = depth[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    // Find the maximum depth
    let max_depth = *depth.iter().max().unwrap();

    // Determine the winner based on the maximum depth
    if max_depth % 2 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
