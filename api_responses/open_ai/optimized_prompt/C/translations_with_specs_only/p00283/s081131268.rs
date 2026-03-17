// This program computes the minimum number of operations required to reduce a target value to zero
// using a set of defined operations. If it is not possible, it indicates that.

use std::collections::HashMap;

fn min_operations(target: usize, operations: &[(usize, usize, usize)]) -> Result<usize, &'static str> {
    // Preconditions: target must be non-negative and operations must be valid
    if target > 100 || operations.len() > 100 {
        return Err("Input values are out of bounds.");
    }

    // Memoization structure to store minimum operations for each state
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    
    // Recursive function to compute minimum operations
    fn compute(target: usize, effect: usize, operations: &[(usize, usize, usize)], memo: &mut HashMap<(usize, usize), usize>) -> usize {
        // If target is zero, no operations are needed
        if target == 0 {
            return 0;
        }
        // If target is negative, return a large number (impossible state)
        if target > 100 {
            return usize::MAX;
        }
        // Check if already computed
        if let Some(&result) = memo.get(&(target, effect)) {
            return result;
        }

        let mut min_ops = usize::MAX;

        // Iterate through each operation
        for &(reduce, op_effect, requirement) in operations {
            // Check if the operation can be applied
            if effect >= requirement {
                let new_target = target.saturating_sub(reduce); // Prevent underflow
                let ops = compute(new_target, op_effect, operations, memo);
                if ops != usize::MAX {
                    min_ops = min_ops.min(1 + ops); // Count this operation
                }
            }
        }

        // Store the result in memoization structure
        memo.insert((target, effect), min_ops);
        min_ops
    }

    // Start the computation with initial effect level of 0
    let result = compute(target, 0, operations, &mut memo);
    
    // Postconditions: Check if we found a valid number of operations
    if result == usize::MAX {
        Err("Not achievable")
    } else {
        Ok(result)
    }
}

// Example usage
fn main() {
    let target = 10;
    let operations = vec![(3, 1, 0), (5, 2, 1), (2, 1, 0)];
    
    match min_operations(target, &operations) {
        Ok(min_ops) => println!("Minimum operations required: {}", min_ops),
        Err(message) => println!("{}", message),
    }
}
