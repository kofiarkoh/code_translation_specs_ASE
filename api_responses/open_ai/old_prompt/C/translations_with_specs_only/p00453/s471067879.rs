use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Read the points and weights
        let mut points = Vec::new();
        for _ in 0..n {
            let point_line = reader.lines().next().unwrap().unwrap();
            let point_data: Vec<i32> = point_line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            points.push(point_data);
        }

        // Initialize cost array with a large constant value
        let inf = i32::MAX; // Represents an unreachable cost
        let mut cost = vec![vec![inf; m + 1]; n];

        // Compute costs based on the distance metric and weights
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    // Assuming a simple distance metric based on coordinates
                    let distance = calculate_distance(&points[i], &points[j]);
                    for k in 1..=m {
                        cost[i][k] = cost[i][k].min(cost[j][k - 1] + distance);
                    }
                }
            }
        }

        // Find the minimum cost for the last valid input set
        let min_cost = cost.iter().flat_map(|c| c.iter()).cloned().min().unwrap_or(inf);
        println!("{}", if min_cost == inf { -1 } else { min_cost });
    }
}

// Function to calculate the distance between two points
fn calculate_distance(point_a: &[i32], point_b: &[i32]) -> i32 {
    // Assuming Euclidean distance for simplicity
    let dx = point_a[0] - point_b[0];
    let dy = point_a[1] - point_b[1];
    (dx * dx + dy * dy).sqrt() as i32 // Cast to i32 for consistency with the specification
}
