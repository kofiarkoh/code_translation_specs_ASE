use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Node {
    depth: usize,
    cost: usize,
}

fn main() {
    // Read the probability factor
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let probability_factor: f64 = input.trim().parse().expect("Invalid probability factor");

    // Read the number of nodes
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_nodes: usize = input.trim().parse().expect("Invalid number of nodes");

    // Initialize the adjacency list for the tree
    let mut tree: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    // Read the edges
    for _ in 0..(num_nodes - 1) {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();
        let (node1, node2, cost) = (edge[0], edge[1], edge[2]);
        tree.entry(node1).or_default().push((node2, cost));
        tree.entry(node2).or_default().push((node1, cost));
    }

    // Calculate the weighted sum
    let weighted_sum = calculate_weighted_sum(&tree, probability_factor, num_nodes);
    
    // Output the result formatted to ten decimal places
    println!("{:.10}", weighted_sum);
}

fn calculate_weighted_sum(tree: &HashMap<usize, Vec<(usize, usize)>>, probability_factor: f64, num_nodes: usize) -> f64 {
    let mut visited = vec![false; num_nodes];
    let mut total_sum = 0.0;

    // Start DFS from the root node (assumed to be node 0)
    dfs(0, 0, probability_factor, tree, &mut visited, &mut total_sum);

    total_sum
}

fn dfs(node: usize, depth: usize, probability_factor: f64, tree: &HashMap<usize, Vec<(usize, usize)>>, visited: &mut Vec<bool>, total_sum: &mut f64) {
    visited[node] = true;

    // Calculate the contribution of the current node
    let cost = tree.get(&node).unwrap_or(&vec![]).iter().map(|&(_, cost)| cost).sum::<usize>();
    *total_sum += (cost as f64) * (depth as f64) * probability_factor;

    // Visit all connected nodes
    for &(neighbor, _) in &tree[&node] {
        if !visited[neighbor] {
            dfs(neighbor, depth + 1, probability_factor, tree, visited, total_sum);
        }
    }
}
