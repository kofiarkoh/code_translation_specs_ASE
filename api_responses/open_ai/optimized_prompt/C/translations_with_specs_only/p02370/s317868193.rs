use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of vertices and edges
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let num_vertices: usize = parts.next().unwrap().parse().unwrap();
    let num_edges: usize = parts.next().unwrap().parse().unwrap();

    // Initialize adjacency list and incoming edge count
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    // Read edges and populate the graph
    for _ in 0..num_edges {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let from: usize = edge_parts.next().unwrap().parse().unwrap();
        let to: usize = edge_parts.next().unwrap().parse().unwrap();

        // Add edge to adjacency list
        adj_list.entry(from).or_insert_with(Vec::new).push(to);
        // Increment in-degree for the destination vertex
        *in_degree.entry(to).or_insert(0) += 1;
        // Ensure the source vertex is also in the in_degree map
        in_degree.entry(from).or_insert(0);
    }

    // Queue for vertices with no incoming edges
    let mut queue: VecDeque<usize> = VecDeque::new();
    for &vertex in in_degree.keys() {
        if in_degree[&vertex] == 0 {
            queue.push_back(vertex);
        }
    }

    // Vector to hold the topological order
    let mut topological_order = Vec::new();

    // Process the graph
    while let Some(vertex) = queue.pop_front() {
        topological_order.push(vertex);
        // Decrease the in-degree of adjacent vertices
        if let Some(neighbors) = adj_list.get(&vertex) {
            for &neighbor in neighbors {
                let count = in_degree.entry(neighbor).or_insert(0);
                *count -= 1;
                // If in-degree becomes zero, add to queue
                if *count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Output the topological order
    for vertex in topological_order {
        println!("{}", vertex);
    }

    // If the topological order does not contain all vertices, the graph has a cycle
    // This behavior is undefined as per the specification.
}
