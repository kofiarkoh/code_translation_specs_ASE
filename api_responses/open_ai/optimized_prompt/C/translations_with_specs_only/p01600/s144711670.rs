// This Rust program computes the minimum cost to connect a series of points in a 2D space.
// The cost is determined by the absolute differences in coordinates between connected points.

fn min_cost_connect_points(points: Vec<(i32, i32)>) -> i32 {
    let n = points.len();
    if n == 0 {
        return 0; // No points, no cost
    }

    // Initialize the cost matrix with a high value (using i32::MAX)
    let mut cost_matrix = vec![vec![i32::MAX; n]; n];
    
    // Set the diagonal to zero, as the cost to connect a point to itself is zero
    for i in 0..n {
        cost_matrix[i][i] = 0;
    }

    // Fill the cost matrix with the absolute differences in coordinates
    for i in 0..n {
        for j in i + 1..n {
            let cost = (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();
            cost_matrix[i][j] = cost;
            cost_matrix[j][i] = cost; // The cost is symmetric
        }
    }

    // Prim's algorithm to find the minimum spanning tree (MST)
    let mut min_cost = 0;
    let mut visited = vec![false; n];
    let mut edges = vec![(0, 0)]; // (cost, point_index)

    while edges.len() < n {
        // Find the minimum edge from the edges list
        let (cost, point_index) = edges.pop().unwrap();
        if visited[point_index] {
            continue; // Skip if already visited
        }
        visited[point_index] = true;
        min_cost += cost;

        // Add all edges from the current point to the edges list
        for next in 0..n {
            if !visited[next] {
                edges.push((cost_matrix[point_index][next], next));
            }
        }
        // Sort edges to get the minimum cost edge at the top
        edges.sort_by_key(|&(c, _)| c);
    }

    min_cost
}

// Example usage
fn main() {
    let points = vec![(0, 0), (1, 1), (1, 0), (0, 1)];
    let result = min_cost_connect_points(points);
    println!("Minimum cost to connect points: {}", result);
}
