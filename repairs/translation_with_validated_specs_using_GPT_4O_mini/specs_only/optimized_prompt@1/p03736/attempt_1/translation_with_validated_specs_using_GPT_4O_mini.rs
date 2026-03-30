use std::cmp::min;

fn compute_min_cost(n: usize, q: usize, i: usize, j: usize, queries: &[usize]) -> u64 {
    // Preconditions check (not implemented as per specification, assuming valid input)
    // let mut min_costs = vec![u64::MAX; n]; // Initialize to a predefined maximum value
    let mut total_cost: u64 = 0;

    // Initial indices
    let mut current_index = i;
    let mut previous_index = j;

    // Process each query
    for &query in queries.iter() {
        // Check if query index is within bounds
        if query >= n {
            // Undefined behavior as per specification; we will just skip this query
            continue;
        }

        // Calculate the cost based on the distance between indices
        let distance = (current_index as isize - query as isize).abs() as u64;
        total_cost += distance;

        // Update the current index for the next iteration
        previous_index = current_index;
        current_index = query;
    }

    total_cost
}

fn main() {
    // Example usage (not part of the specification, just for demonstration)
    let n = 5;
    let q = 3;
    let i = 0;
    let j = 1;
    let queries = vec![2, 3, 4];

    let result = compute_min_cost(n, q, i, j, &queries);
    println!("{}", result);
}
