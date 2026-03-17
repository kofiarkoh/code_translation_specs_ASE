use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};

fn main() {
    // Read the number of nodes (N) and edges (M)
    let (n, m) = read_input();

    // Construct the graph
    let graph = construct_graph(n, m);

    // Perform BFS starting from node 1
    let parents = bfs(&graph, n);

    // Output the results
    output_results(parents);
}

// Function to read input values for N and M
fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly two inputs for N and M
    if parts.len() != 2 {
        panic!("Invalid input. Expected two integers for N and M.");
    }

    let n = parts[0];
    let m = parts[1];

    // Preconditions: N must be positive and M must be non-negative
    if n == 0 {
        panic!("N must be a positive integer.");
    }

    (n, m)
}

// Function to construct the graph as an adjacency list
fn construct_graph(n: usize, m: usize) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..m {
        let mut edge_input = String::new();
        io::stdin().read_line(&mut edge_input).expect("Failed to read line");
        let edge: Vec<usize> = edge_input.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly two nodes for the edge
        if edge.len() != 2 {
            panic!("Invalid edge input. Expected two integers.");
        }

        let u = edge[0];
        let v = edge[1];

        // Ensure nodes are within the valid range
        if u < 1 || u > n || v < 1 || v > n {
            panic!("Edge connects invalid nodes.");
        }

        // Add the edge to the graph (undirected)
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    graph
}

// Function to perform BFS and determine parent nodes
fn bfs(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Vec<i32> {
    let mut parents = vec![-1; n + 1]; // Initialize parent array with -1
    let mut visited = vec![false; n + 1]; // Track visited nodes
    let mut queue = VecDeque::new();

    // Start BFS from node 1
    queue.push_back(1);
    visited[1] = true;

    while let Some(node) = queue.pop_front() {
        for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
            if !visited[neighbor] {
                visited[neighbor] = true;
                parents[neighbor] = node as i32; // Set parent
                queue.push_back(neighbor);
            }
        }
    }

    parents
}

// Function to output the results
fn output_results(parents: Vec<i32>) {
    // Confirmation message
    println!("BFS completed successfully.");

    // Output parent nodes for each node from 2 to N
    for i in 2..parents.len() {
        if parents[i] != -1 {
            println!("Parent of node {}: {}", i, parents[i]);
        }
    }
}
