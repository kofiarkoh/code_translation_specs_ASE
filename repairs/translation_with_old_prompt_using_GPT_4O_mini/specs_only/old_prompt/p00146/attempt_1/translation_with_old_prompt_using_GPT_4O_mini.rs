use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of tasks
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Early exit for n = 0
    if n == 0 {
        return;
    }

    // Read task identifiers, attributes, and weights
    let a: Vec<i32> = lines.next().unwrap().unwrap().trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let b: Vec<i32> = lines.next().unwrap().unwrap().trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let c: Vec<i32> = lines.next().unwrap().unwrap().trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    // Initialize cost and relationship matrices
    let mut d = vec![vec![f64::INFINITY; 1 << n]; n];
    let mut fr = vec![vec![0; 1 << n]; n];

    // Base case: cost of performing a single task
    for i in 0..n {
        d[i][1 << i] = c[i] as f64; // Cost of performing task i alone
    }

    // Fill the cost matrix using dynamic programming
    for mask in 1..(1 << n) {
        for i in 0..n {
            if mask & (1 << i) == 0 {
                continue; // Task i is not in the current subset
            }
            let prev_mask = mask ^ (1 << i);
            for j in 0..n {
                if prev_mask & (1 << j) == 0 {
                    continue; // Task j is not in the previous subset
                }
                let cost = d[j][prev_mask] + compute_cost(i, j, &b, &c);
                if cost < d[i][mask] {
                    d[i][mask] = cost;
                    fr[i][mask] = j; // Record the predecessor
                }
            }
        }
    }

    // Find the minimum cost and the corresponding last task
    let mut min_cost = f64::INFINITY;
    let mut last_task = 0;
    let full_mask = (1 << n) - 1;
    for i in 0..n {
        if d[i][full_mask] < min_cost {
            min_cost = d[i][full_mask];
            last_task = i;
        }
    }

    // Reconstruct the optimal order of tasks
    let mut order = Vec::new();
    let mut mask = full_mask;
    while mask > 0 {
        order.push(a[last_task]);
        let next_task = fr[last_task][mask];
        mask ^= 1 << last_task; // Remove last_task from the mask
        last_task = next_task; // Move to the predecessor
    }
    order.reverse(); // Reverse to get the correct order

    // Output the optimal order of task identifiers
    println!("{}", order.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

// Helper function to compute the cost between two tasks
fn compute_cost(i: usize, j: usize, b: &[i32], c: &[i32]) -> f64 {
    // Example cost calculation based on attributes and weights
    // This is a placeholder; the actual cost function should be defined based on the specification
    (b[i] - b[j]).abs() as f64 + (c[i] + c[j]) as f64
}
