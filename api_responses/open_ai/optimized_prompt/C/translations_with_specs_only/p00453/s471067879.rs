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

        let mut cost_matrix = vec![vec![i32::MAX; n]; n]; // Initialize cost matrix to high value

        for _ in 0..n {
            let point_count: usize = iter.next().unwrap().parse().unwrap();
            let mut points = Vec::new();

            for _ in 0..point_count {
                let x: i32 = iter.next().unwrap().parse().unwrap();
                let weight: i32 = iter.next().unwrap().parse().unwrap();
                points.push((x, weight));
            }

            // Here we would compute the costs based on the points
            // For simplicity, we assume a direct cost assignment for demonstration
            for i in 0..point_count {
                for j in 0..point_count {
                    if i != j {
                        // Example cost calculation (this should be replaced with actual logic)
                        let cost = points[j].1; // Using weight as cost for demonstration
                        cost_matrix[i][j] = cost_matrix[i][j].min(cost);
                    }
                }
            }
        }

        // Compute the minimum cost from the first to the last point set
        let mut min_cost = i32::MAX;
        for i in 0..n {
            for j in 0..n {
                if cost_matrix[i][j] < min_cost {
                    min_cost = cost_matrix[i][j];
                }
            }
        }

        // Output the minimum cost
        if min_cost == i32::MAX {
            println!("No valid traversal exists");
        } else {
            println!("{}", min_cost);
        }
    }
}
