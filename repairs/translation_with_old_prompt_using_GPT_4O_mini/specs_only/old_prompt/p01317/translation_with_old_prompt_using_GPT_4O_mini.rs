use std::io::{self, BufRead};

const INF: i32 = i32::MAX; // Constant to represent no valid path

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N (number of nodes), M (number of edges), R (number of specified nodes to visit)
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let m: usize = params.next().unwrap().parse().unwrap();
    let r: usize = params.next().unwrap().parse().unwrap();

    // Initialize distance matrices
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0; // Distance to self is zero
    }

    // Read edges and populate the distance matrix
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_params = edge_line.split_whitespace();
        let x: usize = edge_params.next().unwrap().parse().unwrap();
        let y: usize = edge_params.next().unwrap().parse().unwrap();
        let s: i32 = edge_params.next().unwrap().parse().unwrap();
        let _t: char = edge_params.next().unwrap().chars().next().unwrap(); // Edge type is read but not used

        // Update the distance matrix for the edge
        dist[x][y] = dist[x][y].min(s);
        dist[y][x] = dist[y][x].min(s); // Assuming undirected graph
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != INF && dist[k][j] != INF {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Read the specified nodes to visit
    let mut visit_nodes = Vec::new();
    for _ in 0..r {
        let visit_line = lines.next().unwrap().unwrap();
        let visit_node: usize = visit_line.trim().parse().unwrap();
        visit_nodes.push(visit_node);
    }

    // Dynamic programming to find the minimum cost to visit all specified nodes
    let mut dp = vec![vec![INF; 1 << r]; r];
    for i in 0..r {
        dp[i][1 << i] = 0; // Starting at each node with only that node visited
    }

    for mask in 1..(1 << r) {
        for u in 0..r {
            if (mask & (1 << u)) == 0 {
                continue; // Node u is not visited in this mask
            }
            for v in 0..r {
                if u == v || (mask & (1 << v)) != 0 {
                    continue; // Skip if v is the same as u or already visited
                }
                let next_mask = mask | (1 << v);
                let cost = dp[u][mask] + dist[visit_nodes[u]][visit_nodes[v]];
                dp[v][next_mask] = dp[v][next_mask].min(cost);
            }
        }
    }

    // Find the minimum cost to visit all nodes
    let final_mask = (1 << r) - 1;
    let min_cost = (0..r).map(|i| dp[i][final_mask]).min().unwrap_or(INF);

    // Output the result
    if min_cost == INF {
        println!("No valid path exists");
    } else {
        println!("{}", min_cost);
    }
}
