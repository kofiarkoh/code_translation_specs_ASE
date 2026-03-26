use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of nodes and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Create adjacency list for the directed graph
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    // Read edges
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edge_parts = line.split_whitespace();
        let u: usize = edge_parts.next().unwrap().parse().unwrap();
        let v: usize = edge_parts.next().unwrap().parse().unwrap();
        graph[u].insert(v);
    }

    // Prepare to check for multiple paths
    let mut visited = vec![false; n];
    let mut output = vec!["same"; m];

    // Function to perform DFS and count paths
    fn dfs(node: usize, graph: &Vec<HashSet<usize>>, visited: &mut Vec<bool>, path_count: &mut usize) {
        visited[node] = true;
        for &neighbor in &graph[node] {
            *path_count += 1; // Count the path
            if !visited[neighbor] {
                dfs(neighbor, graph, visited, path_count);
            }
        }
    }

    // Check each edge for multiple paths
    for (i, edge) in graph.iter().enumerate() {
        let mut path_count = 0;
        visited.fill(false); // Reset visited for each edge check
        dfs(i, &graph, &mut visited, &mut path_count);
        
        // If more than one path exists, mark as "diff"
        if path_count > 1 {
            output[i] = "diff";
        }
    }

    // Print results
    for result in output {
        println!("{}", result);
    }
}
