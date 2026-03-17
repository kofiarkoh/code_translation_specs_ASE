use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for number of nodes and edges
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    
    let n: usize = parts.next().unwrap().parse().expect("Invalid number of nodes");
    let m: usize = parts.next().unwrap().parse().expect("Invalid number of edges");

    // Initialize parent array and incoming edges count
    let mut parents = vec![0; n + 1]; // Index 0 is unused
    let mut incoming_edges = vec![0; n + 1]; // Index 0 is unused
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let from: usize = edge_parts.next().unwrap().parse().expect("Invalid edge from");
        let to: usize = edge_parts.next().unwrap().parse().expect("Invalid edge to");

        // Add edge to the graph
        graph.entry(from).or_insert_with(HashSet::new).insert(to);
        incoming_edges[to] += 1;
    }

    // Process nodes to find parents in topological order
    let mut no_incoming = Vec::new();
    for node in 1..=n {
        if incoming_edges[node] == 0 {
            no_incoming.push(node);
        }
    }

    while let Some(node) = no_incoming.pop() {
        // For each child of the current node, update its parent and incoming edge count
        if let Some(children) = graph.get(&node) {
            for &child in children {
                parents[child] = node; // Set parent
                incoming_edges[child] -= 1; // Remove incoming edge
                if incoming_edges[child] == 0 {
                    no_incoming.push(child); // Add child to processing queue
                }
            }
        }
    }

    // Output the parent nodes
    for i in 1..=n {
        println!("{}", parents[i]);
    }
}
