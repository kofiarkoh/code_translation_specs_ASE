fn compute_min_cost(n: usize, m: usize, adjustments: &[i32], inputs: &[i32]) -> i32 {
    // Define a large constant to represent an invalid transformation
    const INVALID_TRANSFORMATION: i32 = i32::MAX;

    // Ensure the input sizes are valid
    if n == 0 || m == 0 || adjustments.len() != m || inputs.len() != n {
        return INVALID_TRANSFORMATION;
    }

    // Initialize a cost matrix with a size of n x m
    let mut cost_matrix = vec![vec![INVALID_TRANSFORMATION; m]; n];

    // Fill the cost matrix based on the transformation costs
    for i in 0..n {
        for j in 0..m {
            // Calculate the transformed value
            let transformed_value = inputs[i] + adjustments[j];

            // Check if the transformed value is valid (within a specified range)
            // Assuming the valid range is defined as [0, i32::MAX]
            if transformed_value >= 0 {
                // Update the cost matrix with the transformation cost
                cost_matrix[i][j] = transformed_value; // Cost is the transformed value itself
            }
        }
    }

    // Calculate the minimum cost for transforming all input values
    let mut min_cost = INVALID_TRANSFORMATION;

    for j in 0..m {
        let mut total_cost = 0;
        for i in 0..n {
            total_cost += cost_matrix[i][j];
            // If any cost is invalid, break early
            if total_cost >= INVALID_TRANSFORMATION {
                total_cost = INVALID_TRANSFORMATION;
                break;
            }
        }
        // Update the minimum cost if a valid total cost is found
        if total_cost < min_cost {
            min_cost = total_cost;
        }
    }

    // Return the minimum cost or the large constant if no valid transformation exists
    if min_cost == INVALID_TRANSFORMATION {
        INVALID_TRANSFORMATION
    } else {
        min_cost
    }
}
