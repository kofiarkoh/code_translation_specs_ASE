use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of vertices (V) and edges (E)
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    // If there are no vertices, terminate without output
    if v == 0 {
        return;
    }

    // Initialize adjacency list and in-degree counts
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: Vec<usize> = vec![0; v];

    // Read edges and populate the adjacency list and in-degrees
    for _ in 0..e {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let t: usize = edge_parts.next().unwrap().parse().unwrap();

        // Add the edge to the adjacency list
        adj_list.entry(s).or_insert_with(Vec::new).push(t);
        // Increment the in-degree of the target vertex
        in_degree[t] += 1;
    }

    // Perform topological sort using Kahn's algorithm
    let mut zero_in_degree: HashSet<usize> = (0..v).filter(|&i| in_degree[i] == 0).collect();
    let mut sorted_order: Vec<usize> = Vec::new();

    while !zero_in_degree.is_empty() {
        // Get a vertex with zero in-degree
        let current = *zero_in_degree.iter().next().unwrap();
        zero_in_degree.remove(&current);
        sorted_order.push(current);

        // Decrease the in-degree of adjacent vertices
        if let Some(neighbors) = adj_list.get(&current) {
            for &neighbor in neighbors {
                in_degree[neighbor] -= 1;
                // If in-degree becomes zero, add to the set
                if in_degree[neighbor] == 0 {
                    zero_in_degree.insert(neighbor);
                }
            }
        }
    }

    // Output the sorted order, each vertex on a new line
    for vertex in sorted_order {
        println!("{}", vertex);
    }
}
