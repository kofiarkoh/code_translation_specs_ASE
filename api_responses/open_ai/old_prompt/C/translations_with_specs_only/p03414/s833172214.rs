use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes and edges
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0]; // number of nodes
    let m = parts[1]; // number of edges

    // Create adjacency list for the directed graph
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let edge_parts: Vec<usize> = edge_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let u = edge_parts[0]; // from node
        let v = edge_parts[1]; // to node
        graph[u].insert(v);
    }

    // To store the output for each edge
    let mut output = Vec::new();

    // Check for multiple paths using DFS
    for u in 0..n {
        for &v in &graph[u] {
            let mut visited = vec![false; n];
            let mut path_count = 0;

            // Perform DFS to count paths from u to v
            dfs(u, v, &graph, &mut visited, &mut path_count);

            // If more than one path exists, mark as "diff", otherwise "same"
            if path_count > 1 {
                output.push("diff".to_string());
            } else {
                output.push("same".to_string());
            }
        }
    }

    // Print the output
    for result in output {
        println!("{}", result);
    }
}

// Depth-first search to count paths from start to end
fn dfs(start: usize, end: usize, graph: &Vec<HashSet<usize>>, visited: &mut Vec<bool>, path_count: &mut usize) {
    if start == end {
        *path_count += 1;
        return;
    }

    visited[start] = true;

    for &neighbor in &graph[start] {
        if !visited[neighbor] {
            dfs(neighbor, end, graph, visited, path_count);
        }
    }

    visited[start] = false; // Backtrack
}
