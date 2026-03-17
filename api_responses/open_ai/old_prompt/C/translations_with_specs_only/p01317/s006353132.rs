fn main() {
    // Read input values for N, M, R
    let (n, m, r): (usize, usize, usize) = read_input(); // Assume read_input() is defined to read the required inputs

    // Initialize distance matrices with infinity
    const INF: i32 = i32::MAX; // Use i32::MAX to represent infinity
    let mut dist = vec![vec![INF; n]; n];

    // Read edges and populate the distance matrix
    for _ in 0..m {
        let (x, y, s, t): (usize, usize, i32, char) = read_edge(); // Assume read_edge() is defined to read edges
        dist[x][y] = dist[x][y].min(s); // Store the minimum cost for direct roads or shortcuts
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

    // Read the specified nodes to visit in order
    let specified_nodes: Vec<usize> = read_specified_nodes(r); // Assume read_specified_nodes() is defined

    // Dynamic programming to find the minimum cost to visit specified nodes
    let mut dp = vec![vec![INF; 1 << r]; r]; // DP table for minimum costs
    dp[0][1] = 0; // Starting point

    for mask in 0..(1 << r) {
        for i in 0..r {
            if mask & (1 << i) == 0 { continue; } // If node i is not visited, skip
            for j in 0..r {
                if mask & (1 << j) != 0 { continue; } // If node j is already visited, skip
                let next_mask = mask | (1 << j);
                let cost = dp[i][mask] + dist[specified_nodes[i]][specified_nodes[j]];
                dp[j][next_mask] = dp[j][next_mask].min(cost);
            }
        }
    }

    // Find the minimum cost to visit all specified nodes
    let final_mask = (1 << r) - 1;
    let min_cost = (0..r).map(|i| dp[i][final_mask]).min().unwrap_or(INF);

    // Output the result
    if min_cost == INF {
        println!("-1"); // If no valid path exists, output -1
    } else {
        println!("{}", min_cost); // Output the minimum cost
    }
}

// Placeholder functions for reading input
fn read_input() -> (usize, usize, usize) {
    // Implementation to read N, M, R
    unimplemented!()
}

fn read_edge() -> (usize, usize, i32, char) {
    // Implementation to read x, y, s, t
    unimplemented!()
}

fn read_specified_nodes(r: usize) -> Vec<usize> {
    // Implementation to read the specified nodes
    unimplemented!()
}
