fn compute_min_cost(n: usize, q: usize, mut i: usize, mut j: usize, queries: &[usize]) -> i64 {
    // Preconditions check
    assert!(n > 0, "n must be a positive integer");
    assert!(q >= 0, "q must be a non-negative integer");
    assert!(i < n, "initial index i must be within bounds");
    assert!(j < n, "initial index j must be within bounds");
    
    // Initialize the minimum values to a predefined maximum value
    let mut min_values = vec![i64::MAX; n];
    min_values[i] = 0; // Starting point for the initial index i
    min_values[j] = 0; // Starting point for the initial index j

    let mut total_cost = 0;

    // Process each query
    for &query in queries.iter() {
        assert!(query < n, "query index must be within bounds");
        
        // Calculate the distance from the current indices to the query index
        let distance_i = (query as i64 - i as i64).abs();
        let distance_j = (query as i64 - j as i64).abs();
        
        // Update the total cost with the minimum distance to the query index
        total_cost += distance_i.min(distance_j);
        
        // Update the current indices to the query index
        i = query; // Move i to the current query index
        j = query; // Move j to the current query index
        
        // Update the minimum values based on the current index
        min_values[query] = min_values[query].min(total_cost);
    }

    // Return the total cost as the minimum cost after processing all queries
    total_cost
}

fn main() {
    // Read input values
    let n: usize = 5; // Size of the array
    let q: usize = 3; // Number of queries
    let i: usize = 1; // Initial index i
    let j: usize = 3; // Initial index j
    let queries = vec![2, 4, 5]; // Queries

    // Compute the minimum cost
    let result = compute_min_cost(n, q, i, j, &queries);
    
    // Print the result
    println!("{}", result); // Output the minimum cost directly
}
