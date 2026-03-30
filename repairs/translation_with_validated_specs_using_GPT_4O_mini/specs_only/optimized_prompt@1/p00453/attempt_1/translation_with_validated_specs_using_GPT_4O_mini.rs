use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        let mut points = Vec::new();

        for _ in 0..n {
            let point_data: Vec<i32> = iter.by_ref().take(2).map(|s| s.parse().unwrap()).collect();
            let num_coords = point_data[0] as usize;
            let mut coords_and_weights = Vec::new();

            for _ in 0..num_coords {
                let coord: i32 = iter.next().unwrap().parse().unwrap();
                let weight: i32 = iter.next().unwrap().parse().unwrap();
                coords_and_weights.push((coord, weight));
            }

            points.push(coords_and_weights);
        }

        // Initialize cost matrix with a high value
        let inf = i32::MAX;
        let mut cost_matrix = vec![vec![inf; n]; n];

        // Calculate costs based on the given formula
        for i in 0..n {
            for j in i + 1..n {
                for &(coord1, weight1) in &points[i] {
                    for &(coord2, weight2) in &points[j] {
                        let cost = (weight1 + weight2) * (coord1 - coord2).abs();
                        cost_matrix[i][j] = cost_matrix[i][j].min(cost);
                        cost_matrix[j][i] = cost_matrix[i][j]; // Symmetric cost
                    }
                }
            }
        }

        // Compute minimum cost using dynamic programming
        let mut min_cost = inf;

        // Iterate over all possible segments
        for segments in 1..=m {
            let mut dp = vec![inf; n];
            dp[0] = 0; // Starting point

            for i in 0..n {
                for j in i + 1..n {
                    if segments > 1 {
                        dp[j] = dp[j].min(dp[i] + cost_matrix[i][j]);
                    }
                }
            }

            min_cost = min_cost.min(dp[n - 1]);
        }

        println!("{}", min_cost);
    }
}
