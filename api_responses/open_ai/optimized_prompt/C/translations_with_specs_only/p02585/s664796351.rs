fn compute_max_value(n: usize, k: usize, p: &[usize], c: &[i64]) -> i64 {
    // Preconditions check
    if n == 0 || k < 0 || p.len() != n || c.len() != n {
        // Behavior is not specified for invalid inputs, so we return 0 as a safe default.
        return 0;
    }

    let mut max_value = 0;

    // Iterate through all possible operations up to K
    for operations in 0..=k {
        // Temporary value to hold the sum of costs for the current number of operations
        let mut current_value = 0;

        // Iterate through the positions array
        for i in 0..n {
            // Check if the position is a valid index
            if p[i] < n {
                // Accumulate the cost for the valid position
                current_value += c[p[i]];
            }
        }

        // Update the maximum value if the current value is greater
        if current_value > max_value {
            max_value = current_value;
        }
    }

    // Postcondition: The output value is guaranteed to be less than or equal to the sum of costs in C
    let sum_of_costs: i64 = c.iter().sum();
    if max_value > sum_of_costs {
        max_value = sum_of_costs; // Ensure we adhere to the invariant
    }

    max_value
}

fn main() {
    // Example usage
    let n = 5;
    let k = 3;
    let p = vec![0, 1, 2, 3, 4];
    let c = vec![10, 20, 30, 40, 50];

    let result = compute_max_value(n, k, &p, &c);
    println!("Maximum computed value: {}", result);
}
