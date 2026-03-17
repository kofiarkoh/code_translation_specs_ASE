fn main() {
    // Example input: first integer is the count of subsequent integers,
    // second integer is the count of weights, followed by the integers and weights.
    let input = vec![5, 3, 10, 20, 30, 5, 15, 25]; // Example input for testing
    let result = process_input(&input);
    println!("{}", result);
}

fn process_input(input: &[usize]) -> i32 {
    // Ensure the input has at least two integers for counts
    if input.len() < 2 {
        return -1; // Not enough input
    }

    let count_integers = input[0];
    let count_weights = input[1];

    // Validate counts
    if count_integers == 0 || count_weights == 0 || input.len() < (2 + count_integers + count_weights) {
        return -1; // Invalid counts or insufficient input
    }

    // Collect integers and weights from the input
    let integers: Vec<usize> = input[2..2 + count_integers].to_vec();
    let weights: Vec<usize> = input[2 + count_integers..2 + count_integers + count_weights].to_vec();

    // Set to hold computed values derived from weights
    let mut computed_values = std::collections::HashSet::new();

    // Compute values based on weights
    for &weight in &weights {
        computed_values.insert(weight); // Store each weight as a computed value
    }

    // Initialize minimum difference to a large value
    let mut min_difference = usize::MAX;

    // Check each integer against the computed values
    for &integer in &integers {
        for &computed in &computed_values {
            let difference = integer.abs_diff(computed); // Calculate absolute difference
            if difference < min_difference {
                min_difference = difference; // Update minimum difference
            }
        }
    }

    // If min_difference was updated, return it; otherwise, return -1
    if min_difference == usize::MAX {
        -1 // No valid difference found
    } else {
        min_difference as i32 // Return the minimum difference as i32
    }
}

// Extension trait to provide abs_diff for usize
trait AbsDiff {
    fn abs_diff(self, other: Self) -> Self;
}

impl AbsDiff for usize {
    fn abs_diff(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}
