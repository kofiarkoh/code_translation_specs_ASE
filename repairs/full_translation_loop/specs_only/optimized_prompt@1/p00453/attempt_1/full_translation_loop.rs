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

        let mut total_cost = 0;

        for _ in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let mut point_iter = line.split_whitespace();
            let num_points: usize = point_iter.next().unwrap().parse().unwrap();

            // Initialize cost array with a high value
            let mut costs = vec![i32::MAX; num_points];

            for i in 0..num_points {
                let x: i32 = point_iter.next().unwrap().parse().unwrap();
                let weight: i32 = point_iter.next().unwrap().parse().unwrap();
                // Assuming the cost is simply the weight for this point
                costs[i] = weight;
            }

            // Calculate minimum cost based on the weights
            for i in 0..num_points {
                if costs[i] < total_cost {
                    total_cost = costs[i];
                }
            }
        }

        // Output the minimum cost for the current dataset
        println!("{}", total_cost);
    }
}
