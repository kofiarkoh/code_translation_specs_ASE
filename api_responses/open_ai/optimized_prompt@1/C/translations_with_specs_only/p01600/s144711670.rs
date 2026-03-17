// This Rust program computes the minimum cost to connect a series of points in a 2D space.
// The cost is determined by the absolute differences in coordinates between connected points.

fn min_cost_to_connect_points(points: Vec<(i32, i32)>) -> i32 {
    let n = points.len();
    
    // If there are no points, the cost is 0.
    if n == 0 {
        return 0;
    }

    // Create a distance matrix to store the costs between points.
    let mut dist = vec![vec![0; n]; n];

    // Fill the distance matrix with the absolute differences in coordinates.
    for i in 0..n {
        for j in 0..n {
            dist[i][j] = (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();
        }
    }

    // Initialize a DP table to store the minimum costs for connecting subsets of points.
    let mut dp = vec![i32::MAX; 1 << n];
    dp[0] = 0; // Cost of connecting no points is 0.

    // Iterate over all subsets of points.
    for mask in 0..(1 << n) {
        // Count how many points are in the current subset.
        let count = mask.count_ones() as usize;

        // If there are no points in the subset, continue.
        if count == 0 {
            continue;
        }

        // Try to connect a new point to the current subset.
        for j in 0..n {
            // Check if point j is already in the subset.
            if mask & (1 << j) != 0 {
                continue;
            }

            // Calculate the new mask with point j added.
            let new_mask = mask | (1 << j);
            // Update the DP table with the minimum cost to connect the new subset.
            dp[new_mask] = dp[new_mask].min(dp[mask] + dist[mask.trailing_zeros() as usize][j]);
        }
    }

    // The minimum cost to connect all points is stored in dp[(1 << n) - 1].
    dp[(1 << n) - 1]
}

// Example usage (uncomment to test):
// fn main() {
//     let points = vec![(0, 0), (1, 1), (1, 0), (0, 1)];
//     let cost = min_cost_to_connect_points(points);
//     println!("Minimum cost to connect points: {}", cost);
// }
