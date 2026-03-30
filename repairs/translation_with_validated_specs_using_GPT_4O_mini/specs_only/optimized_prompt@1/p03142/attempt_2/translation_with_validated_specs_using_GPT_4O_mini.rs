use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N and M
    let first_line = lines.next().expect("Failed to read N and M").unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().expect("Invalid input for N").parse().expect("N must be an integer");
    let m: usize = parts.next().expect("Invalid input for M").parse().expect("M must be an integer");

    // Initialize graph structures
    let mut sansyo = vec![0; n]; // Incoming edge count for each node
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new(); // Adjacency list

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().expect("Failed to read edge").unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().expect("Invalid input for A").parse().expect("A must be an integer");
        let b: usize = edge_parts.next().expect("Invalid input for B").parse().expect("B must be an integer");

        // Adjust for 0-based indexing
        let a = a - 1;
        let b = b - 1;

        // Add edge to the graph
        graph.entry(a).or_insert_with(Vec::new).push(b);
        sansyo[b] += 1; // Increment incoming edge count for node B
    }

    // Prepare to process nodes with no dependencies
    let mut next_buffer: Vec<usize> = Vec::new();
    for i in 0..n {
        if sansyo[i] == 0 {
            next_buffer.push(i);
        }
    }

    // Output the topological order
    let mut output = Vec::new();
    while !next_buffer.is_empty() {
        let mut current_buffer: Vec<usize> = Vec::new();
        for &node in &next_buffer {
            output.push(node); // Keep 0-based indexing for output
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    sansyo[neighbor] -= 1; // Resolve dependency
                    if sansyo[neighbor] == 0 {
                        current_buffer.push(neighbor);
                    }
                }
            }
        }
        next_buffer = current_buffer; // Move to the next set of nodes with no dependencies
    }

    // Print the output
    for node in output {
        println!("{}", node + 1); // Convert back to 1-based indexing for output
    }
}
