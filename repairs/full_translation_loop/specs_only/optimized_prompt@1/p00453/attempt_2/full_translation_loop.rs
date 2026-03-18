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

        let mut total_cost = i32::MAX; // Initialize to a high value

        for _ in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let mut point_iter = line.split_whitespace();
            let num_points: usize = point_iter.next().unwrap().parse().unwrap();

            // Initialize cost array with a high value
            let mut costs = vec![i32::MAX; num_points];

            for i in 0..num_points {
                let _x: i32 = point_iter.next().unwrap().parse().unwrap(); // x is unused
                let weight: i32 = point_iter.next().unwrap().parse().unwrap();
                // Store the weight as the cost for this point
                costs[i] = weight;
            }

            // Calculate minimum cost based on the weights
            for &cost in &costs {
                if cost < total_cost {
                    total_cost = cost;
                }
            }
        }

        // Output the minimum cost for the current dataset
        println!("{}", total_cost);
    }
}
