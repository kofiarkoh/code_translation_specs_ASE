fn main() {
    // Example input values; in a real scenario, these would be read from input.
    let n: usize = 4; // Number of vertices
    let m: usize = 5; // Number of edges
    let a = vec![0, 0, 1, 2, 3]; // Starting vertices of edges
    let b = vec![1, 2, 2, 3, 0]; // Ending vertices of edges
    let c = vec![1, 2, 1, 3, 4]; // Weights of edges

    // Initialize the distance matrix with "infinity" for unreachable pairs
    let inf = i32::MAX; // Using i32::MAX to represent infinity
    let mut dist = vec![vec![inf; n]; n];

    // Set the distance from each vertex to itself to 0
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Fill the distance matrix with the edge weights
    for i in 0..m {
        dist[a[i]][b[i]] = dist[a[i]][b[i]].min(c[i]);
        dist[b[i]][a[i]] = dist[b[i]][a[i]].min(c[i]); // Undirected graph
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != inf && dist[k][j] != inf {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Count edges not part of any shortest path
    let mut count = 0;
    for i in 0..m {
        let d1 = dist[a[i]][b[i]];
        let d2 = dist[b[i]][a[i]];
        let d3 = dist[a[i]][b[i]];
        if d1 != d3 && d2 != d3 {
            count += 1; // Edge is not part of any shortest path
        }
    }

    // Output the result
    println!("{}", count);
}
