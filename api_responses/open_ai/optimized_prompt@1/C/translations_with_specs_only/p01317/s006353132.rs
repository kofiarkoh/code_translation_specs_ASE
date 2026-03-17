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

struct Graph {
    nodes: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(nodes: usize, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }

    fn min_cost(&self, visit_sequence: &[usize]) -> u32 {
        let mut dist = vec![vec![u32::MAX; self.nodes]; self.nodes];

        // Initialize distances based on edges
        for edge in &self.edges {
            dist[edge.from_node][edge.to_node] = edge.cost;
            // For undirected edges, uncomment the next line
            // dist[edge.to_node][edge.from_node] = edge.cost;
        }

        // Floyd-Warshall algorithm to compute shortest paths
        for k in 0..self.nodes {
            for i in 0..self.nodes {
                for j in 0..self.nodes {
                    if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        // Dynamic programming to find minimum cost to visit the sequence of nodes
        let r = visit_sequence.len();
        let mut dp = vec![u32::MAX; r];

        dp[0] = 0; // Starting point, cost to visit the first node is 0

        for i in 0..r - 1 {
            for j in i + 1..r {
                let cost = dist[visit_sequence[i]][visit_sequence[j]];
                if cost != u32::MAX {
                    dp[j] = dp[j].min(dp[i] + cost);
                }
            }
        }

        // The last element in dp will have the minimum cost to visit all nodes in order
        dp[r - 1]
    }
}

fn main() {
    // Example usage
    let nodes = 4;
    let edges = vec![
        Edge { from_node: 0, to_node: 1, cost: 1, edge_type: EdgeType::Direct },
        Edge { from_node: 1, to_node: 2, cost: 2, edge_type: EdgeType::Direct },
        Edge { from_node: 0, to_node: 2, cost: 5, edge_type: EdgeType::Indirect },
        Edge { from_node: 2, to_node: 3, cost: 1, edge_type: EdgeType::Direct },
    ];

    let graph = Graph::new(nodes, edges);
    let visit_sequence = vec![0, 2, 3];
    let min_cost = graph.min_cost(&visit_sequence);

    // Output the minimum cost
    println!("Minimum cost to traverse: {}", min_cost);
}
