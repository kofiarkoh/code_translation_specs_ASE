fn bounded_knapsack(n: usize, w: &[usize], v: &[usize], w_limit: usize) -> Result<usize, &'static str> {
    const MEW: usize = 112345; // Size of the ayaneru array
    let mut ayaneru = vec![0; MEW]; // Initialize the ayaneru array with zeros

    // Check for valid input
    if n > 128 || w.len() != n || v.len() != n {
        return Err("error!"); // Return error if input is invalid
    }

    // Process each item
    for i in 0..n {
        let weight = w[i];
        let value = v[i];

        // Update the ayaneru array in reverse to avoid overwriting results from the same iteration
        for j in (weight..=w_limit).rev() {
            ayaneru[j] = ayaneru[j].max(ayaneru[j - weight] + value);
        }
    }

    // Find the maximum value that can be achieved without exceeding the weight limit
    let max_value = ayaneru.iter().max().copied().unwrap_or(0);

    // Check if no valid configuration exists
    if max_value == 0 {
        return Err("error!"); // Return error if no valid configuration exists
    }

    Ok(max_value) // Return the maximum value
}

fn main() {
    // Example usage
    let n = 3; // Number of items
    let w = [10, 20, 30]; // Weights of the items
    let v = [60, 100, 120]; // Values of the items
    let w_limit = 50; // Maximum weight capacity

    match bounded_knapsack(n, &w, &v, w_limit) {
        Ok(max_value) => println!("{}", max_value), // Print the maximum value
        Err(err) => println!("{}", err), // Print error message
    }
}
