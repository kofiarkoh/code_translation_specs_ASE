// Define a constant for the large constant value to return when no valid transformation exists
const LARGE_CONSTANT: i32 = 1_000_000_000;

// Function to compute the minimum transformation cost
fn compute_min_cost(n: usize, m: usize, adjustments: &[i32], inputs: &[i32]) -> i32 {
    // Preconditions: n and m must be non-negative integers
    if n == 0 || m == 0 || adjustments.is_empty() || inputs.is_empty() {
        return LARGE_CONSTANT; // Return large constant if no valid transformation can be achieved
    }

    // Initialize a cost matrix to store the transformation costs
    let mut cost_matrix = vec![vec![0; m]; n];

    // Fill the cost matrix based on the adjustments and inputs
    for i in 0..n {
        for j in 0..m {
            // Calculate the cost of transforming the input value using the adjustment value
            let transformed_value = inputs[i] + adjustments[j];
            // Ensure the transformed value is non-negative
            if transformed_value >= 0 {
                cost_matrix[i][j] = transformed_value; // Store the cost
            } else {
                cost_matrix[i][j] = LARGE_CONSTANT; // Use large constant for invalid transformations
            }
        }
    }

    // Calculate the minimum cost for transforming all input values
    let mut min_cost = LARGE_CONSTANT;
    for j in 0..m {
        let total_cost: i32 = cost_matrix.iter().map(|row| row[j]).sum();
        if total_cost < min_cost {
            min_cost = total_cost; // Update min_cost if a lower total cost is found
        }
    }

    // Return the minimum cost found, or the large constant if no valid transformation exists
    min_cost
}

// Main function to execute the program
fn main() {
    let n = 3;
    let m = 2;
    let adjustments = vec![1, 2];
    let inputs = vec![3, 4, 5];
    let result = compute_min_cost(n, m, &adjustments, &inputs);
    println!("Minimum cost: {}", result);
}
