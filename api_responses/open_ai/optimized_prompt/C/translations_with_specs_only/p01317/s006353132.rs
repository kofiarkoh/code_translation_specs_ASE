use std::collections::HashMap;
use std::cmp::Ordering;

const MAX_COST: i32 = i32::MAX; // Define a maximum cost for unreachable paths

#[derive(Debug)]
struct Edge {
    from_node: usize,
    to_node: usize,
    cost: i32,
    edge_type: EdgeType,
}

#[derive(Debug)]
enum EdgeType {
    Direct,
    Indirect,
}

fn main() {
    // Example input for testing
    let test_cases = vec![
        (4, 5, vec![
            (0, 1, 10, EdgeType::Direct),
            (1, 2, 5, EdgeType::Direct),
            (0, 2, 15, EdgeType::Indirect),
            (2, 3, 10, EdgeType::Direct),
            (1, 3, 20, EdgeType::Indirect),
        ], vec![0, 1, 2, 3]),
    ];

    for (num_nodes, num_edges, edges, visit_sequence) in test_cases {
        let min_cost = compute_min_cost(num_nodes, edges, visit_sequence);
        println!("{}", min_cost);
    }
}

fn compute_min_cost(num_nodes: usize, edges: Vec<(usize, usize, i32, EdgeType)>, visit_sequence: Vec<usize>) -> i32 {
    let mut cost_matrix = vec![vec![MAX_COST; num_nodes]; num_nodes];

    // Initialize the cost matrix based on edges
    for (from, to, cost, edge_type) in edges {
        match edge_type {
            EdgeType::Direct => {
                cost_matrix[from][to] = cost_matrix[from][to].min(cost);
            }
            EdgeType::Indirect => {
                cost_matrix[from][to] = cost_matrix[from][to].min(cost);
            }
        }
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if cost_matrix[i][k] != MAX_COST && cost_matrix[k][j] != MAX_COST {
                    cost_matrix[i][j] = cost_matrix[i][j].min(cost_matrix[i][k] + cost_matrix[k][j]);
                }
            }
        }
    }

    // Calculate the minimum cost to visit the specified nodes in order
    let mut total_cost = 0;
    for window in visit_sequence.windows(2) {
        let from = window[0];
        let to = window[1];
        total_cost += cost_matrix[from][to];
    }

    // If total_cost is still MAX_COST, it means at least one path was not reachable
    if total_cost == MAX_COST {
        return MAX_COST; // Return predefined maximum cost if traversal is not possible
    }

    total_cost
}
