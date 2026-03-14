// Import necessary libraries
use std::io::{self, Write};

const MAX_NODES: usize = 20;
const INF: i32 = i32::MAX; // Represents infinity

fn main() {
    // Read number of nodes and edges
    let (num_nodes, num_edges) = read_graph_parameters();

    // Initialize weight matrix
    let mut weight_matrix = initialize_weight_matrix(num_nodes);

    // Read edges and update weight matrix
    read_edges(num_edges, &mut weight_matrix);

    // Compute shortest paths using Floyd-Warshall algorithm
    floyd_warshall(&mut weight_matrix, num_nodes);

    // Read the final cost parameters
    let (start_node, end_node, specified_cost) = read_cost_parameters();

    // Calculate the adjusted cost
    let adjusted_cost = calculate_adjusted_cost(&weight_matrix, start_node, end_node, specified_cost);

    // Output the adjusted cost
    println!("{}", adjusted_cost);
}

// Function to read the number of nodes and edges
fn read_graph_parameters() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<usize> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Preconditions: number of nodes must be positive, edges must be non-negative
    if params.len() != 2 || params[0] == 0 {
        panic!("Invalid input for number of nodes or edges.");
    }

    (params[0], params[1])
}

// Function to initialize the weight matrix
fn initialize_weight_matrix(num_nodes: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![INF; MAX_NODES]; MAX_NODES];
    for i in 0..num_nodes {
        matrix[i][i] = 0; // Diagonal elements are zero
    }
    matrix
}

// Function to read edges and update the weight matrix
fn read_edges(num_edges: usize, weight_matrix: &mut Vec<Vec<i32>>) {
    for _ in 0..num_edges {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let edge: Vec<usize> = input.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Preconditions: edges must reference valid node indices
        if edge.len() != 4 || edge[0] >= MAX_NODES || edge[1] >= MAX_NODES {
            panic!("Invalid edge input.");
        }

        let (u, v, weight) = (edge[0], edge[1], edge[2] as i32);
        weight_matrix[u][v] = weight_matrix[u][v].min(weight);
        weight_matrix[v][u] = weight_matrix[v][u].min(weight); // Undirected graph
    }
}

// Function to compute shortest paths using Floyd-Warshall algorithm
fn floyd_warshall(weight_matrix: &mut Vec<Vec<i32>>, num_nodes: usize) {
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if weight_matrix[i][k] < INF && weight_matrix[k][j] < INF {
                    weight_matrix[i][j] = weight_matrix[i][j].min(weight_matrix[i][k] + weight_matrix[k][j]);
                }
            }
        }
    }
}

// Function to read the final cost parameters
fn read_cost_parameters() -> (usize, usize, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<usize> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Preconditions: must have exactly four parameters
    if params.len() != 4 || params[0] >= MAX_NODES || params[1] >= MAX_NODES {
        panic!("Invalid input for cost parameters.");
    }

    (params[0], params[1], params[2] as i32)
}

// Function to calculate the adjusted cost
fn calculate_adjusted_cost(weight_matrix: &Vec<Vec<i32>>, start_node: usize, end_node: usize, specified_cost: i32) -> i32 {
    let shortest_path_weight = weight_matrix[start_node][end_node];
    if shortest_path_weight == INF {
        // If no path exists, return the specified cost
        return specified_cost;
    }
    specified_cost - shortest_path_weight
}
