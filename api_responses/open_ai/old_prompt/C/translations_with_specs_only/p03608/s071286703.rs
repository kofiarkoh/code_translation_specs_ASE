use std::cmp;

const INF: i32 = i32::MAX; // Use a large constant to represent unreachable paths

fn main() {
    let (n, m, r) = read_input(); // Read input values
    let r_len = r.len();

    // Initialize distance matrix
    let mut d = vec![vec![INF; n]; n];
    for i in 0..n {
        d[i][i] = 0; // Distance to self is zero
    }

    // Read edges and populate the distance matrix
    for _ in 0..m {
        let (u, v, weight) = read_edge(); // Read each edge
        d[u][v] = cmp::min(d[u][v], weight); // Update distance if a shorter path is found
        d[v][u] = cmp::min(d[v][u], weight); // Since the graph is undirected
    }

    // Apply Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] != INF && d[k][j] != INF {
                    d[i][j] = cmp::min(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }
    }

    // If R is 0, return 0 as no nodes need to be visited
    if r_len == 0 {
        println!("0");
        return;
    }

    // Use DFS to find the minimum distance to visit all specified nodes
    let mut used = vec![false; r_len];
    let result = dfs(&d, &r, &mut used, 0, 0, r_len, 0);
    
    // Print the result
    println!("{}", result);
}

// Function to perform DFS and calculate the minimum distance
fn dfs(d: &Vec<Vec<i32>>, r: &Vec<usize>, used: &mut Vec<bool>, pos: usize, count: usize, r_len: usize, current_dist: i32) -> i32 {
    if count == r_len {
        return current_dist + d[r[pos]][r[0]]; // Return to starting point
    }

    let mut min_dist = INF;
    for i in 0..r_len {
        if !used[i] {
            used[i] = true; // Mark this node as visited
            let new_dist = dfs(d, r, used, i, count + 1, r_len, current_dist + d[r[pos]][r[i]]);
            min_dist = cmp::min(min_dist, new_dist);
            used[i] = false; // Backtrack
        }
    }
    min_dist
}

// Placeholder function to read input values
fn read_input() -> (usize, usize, Vec<usize>) {
    // Implement input reading logic here
    // For now, returning dummy values
    (5, 7, vec![0, 1, 2]) // Example: 5 nodes, 7 edges, visiting nodes 0, 1, 2
}

// Placeholder function to read an edge
fn read_edge() -> (usize, usize, i32) {
    // Implement edge reading logic here
    // For now, returning dummy values
    (0, 1, 10) // Example: edge from node 0 to 1 with weight 10
}
