use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of vertices and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let num_vertices: usize = parts.next().unwrap().parse().unwrap();
    let num_edges: usize = parts.next().unwrap().parse().unwrap();

    // Initialize graph structures
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: Vec<usize> = vec![0; num_vertices]; // Corrected initialization

    // Read edges and build the graph
    for _ in 0..num_edges {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let from: usize = edge_parts.next().unwrap().parse().unwrap();
        let to: usize = edge_parts.next().unwrap().parse().unwrap();

        graph.entry(from).or_insert_with(Vec::new).push(to);
        in_degree[to] += 1; // Increment in-degree for the destination vertex
    }

    // Queue for vertices with no incoming edges
    let mut queue: VecDeque<usize> = VecDeque::new();
    for vertex in 0..num_vertices {
        if in_degree[vertex] == 0 {
            queue.push_back(vertex);
        }
    }

    // Result vector for topological order
    let mut topological_order = Vec::new();

    // Process the graph
    while let Some(vertex) = queue.pop_front() {
        topological_order.push(vertex);

        // Decrease the in-degree of adjacent vertices
        if let Some(neighbors) = graph.get(&vertex) {
            for &neighbor in neighbors {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Output the topological order
    for vertex in topological_order {
        println!("{}", vertex);
    }

    // Note: If the graph contains cycles, the output may be incorrect or undefined.
    // This implementation does not check for cycles explicitly.
}
