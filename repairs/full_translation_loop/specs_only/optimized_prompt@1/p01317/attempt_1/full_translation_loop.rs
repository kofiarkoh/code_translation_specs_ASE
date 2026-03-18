use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
struct Edge {
    from_node: usize,
    to_node: usize,
    cost: u32,
    edge_type: EdgeType,
}

#[derive(Debug)]
enum EdgeType {
    Direct,
    Indirect,
}

fn min_traversal_cost(num_nodes: usize, edges: Vec<Edge>, visit_sequence: Vec<usize>) -> u32 {
    // Initialize distance matrix with maximum possible cost
    let mut dist = vec![vec![u32::MAX; num_nodes]; num_nodes];
    
    // Set distance to self as 0
    for i in 0..num_nodes {
        dist[i][i] = 0;
    }

    // Populate the distance matrix with edge costs
    for edge in edges {
        dist[edge.from_node][edge.to_node] = edge.cost;
        // If the edge is indirect, we can also consider it as a direct path
        if let EdgeType::Indirect = edge.edge_type {
            dist[edge.to_node][edge.from_node] = edge.cost; // Assuming undirected for indirect
        }
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Dynamic programming to find minimum cost to visit the sequence of nodes
    let r = visit_sequence.len();
    let mut dp = vec![u32::MAX; r];
    dp[0] = 0; // Starting point

    for i in 0..r {
        for j in i + 1..r {
            let cost = dist[visit_sequence[i]][visit_sequence[j]];
            if cost != u32::MAX {
                dp[j] = dp[j].min(dp[i] + cost);
            }
        }
    }

    // The last element in dp will have the minimum cost to visit all nodes in the sequence
    dp[r - 1]
}

fn main() {
    // Example usage
    let num_nodes = 4;
    let edges = vec![
        Edge { from_node: 0, to_node: 1, cost: 1, edge_type: EdgeType::Direct },
        Edge { from_node: 1, to_node: 2, cost: 2, edge_type: EdgeType::Direct },
        Edge { from_node: 0, to_node: 2, cost: 5, edge_type: EdgeType::Indirect },
        Edge { from_node: 2, to_node: 3, cost: 1, edge_type: EdgeType::Direct },
    ];
    let visit_sequence = vec![0, 2, 3];

    let cost = min_traversal_cost(num_nodes, edges, visit_sequence);
    println!("Minimum traversal cost: {}", cost);
}
