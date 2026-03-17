use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of tasks
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // If n is 0, we do not output anything
    if n == 0 {
        return;
    }

    // Initialize arrays for task identifiers, attributes, and weights
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];

    // Read task identifiers
    let a_input: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    a.copy_from_slice(&a_input);

    // Read task attributes
    let b_input: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    b.copy_from_slice(&b_input);

    // Read task weights
    let c_input: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    c.copy_from_slice(&c_input);

    // Initialize cost and relationship matrices
    let mut d = vec![vec![0.0; 1 << n]; n];
    let mut fr = vec![vec![0; 1 << n]; n];

    // Compute the cost matrix using dynamic programming
    for mask in 0..(1 << n) {
        for i in 0..n {
            if mask & (1 << i) == 0 {
                continue; // Task i is not included in the current subset
            }
            let prev_mask = mask ^ (1 << i);
            for j in 0..n {
                if prev_mask & (1 << j) == 0 {
                    continue; // Task j is not included in the previous subset
                }
                let cost = compute_cost(i, j, &b, &c);
                let new_cost = d[j][prev_mask] + cost;
                if new_cost < d[i][mask] || d[i][mask] == 0.0 {
                    d[i][mask] = new_cost;
                    fr[i][mask] = j; // Store the predecessor
                }
            }
        }
    }

    // Find the optimal order of tasks
    let mut mask = (1 << n) - 1; // All tasks included
    let mut order = Vec::new();
    let mut last_task = 0;

    // Find the task with the minimum cost in the last mask
    for i in 0..n {
        if d[i][mask] < d[last_task][mask] || order.is_empty() {
            last_task = i;
        }
    }

    // Reconstruct the order of tasks
    while mask > 0 {
        order.push(a[last_task]);
        let next_task = fr[last_task][mask];
        mask ^= 1 << last_task; // Remove last_task from the mask
        last_task = next_task; // Move to the predecessor
    }

    // Output the optimal order of task identifiers
    order.reverse(); // Reverse to get the correct order
    println!("{}", order.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

// Helper function to compute the cost between two tasks
fn compute_cost(i: usize, j: usize, b: &[i32], c: &[i32]) -> f64 {
    // Example cost calculation based on attributes and weights
    // This is a placeholder; the actual cost function should be defined based on the specification
    (b[i] - b[j]).abs() as f64 + (c[i] + c[j]) as f64
}
