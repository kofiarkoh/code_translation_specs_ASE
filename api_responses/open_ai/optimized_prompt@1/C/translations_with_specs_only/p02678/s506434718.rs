use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};

fn main() {
    // Read the number of nodes (N) and edges (M)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid number of nodes");
    let m: usize = parts.next().unwrap().parse().expect("Invalid number of edges");

    // Initialize the graph as an adjacency list
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    
    // Read the edges
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let edge: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().expect("Invalid edge"))
            .collect();
        let u = edge[0];
        let v = edge[1];

        // Add the edge in both directions to maintain undirected nature
        graph.entry(u).or_insert_with(HashSet::new).insert(v);
        graph.entry(v).or_insert_with(HashSet::new).insert(u);
    }

    // Initialize the parent array and distance array
    let mut parent: Vec<Option<usize>> = vec![None; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];

    // Perform BFS starting from node 1
    let mut queue = VecDeque::new();
    queue.push_back(1);
    visited[1] = true;

    while let Some(node) = queue.pop_front() {
        for &neighbor in graph.get(&node).unwrap_or(&HashSet::new()) {
            if !visited[neighbor] {
                visited[neighbor] = true;
                parent[neighbor] = Some(node);
                queue.push_back(neighbor);
            }
        }
    }

    // Output the results
    println!("Yes");
    for i in 2..=n {
        match parent[i] {
            Some(p) => print!("{}\n", p),
            None => print!("-1\n"), // If a node is not reachable, we could output -1 or similar
        }
    }
}
