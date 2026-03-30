fn max_allocation(r: u64, b: u64) -> u64 {
    // Binary search for the maximum feasible allocation k
    let mut low = 0;
    let mut high = r + b; // The maximum possible allocation cannot exceed the total resources

    while low < high {
        let mid = (low + high + 1) / 2; // Upper mid to favor higher k in case of ties
        if is_possible(mid, r, b) {
            low = mid; // mid is feasible, search in the upper half
        } else {
            high = mid - 1; // mid is not feasible, search in the lower half
        }
    }

    low.saturating_sub(1) // Return k - 1, ensuring it does not go below zero
}

// Check if a given allocation k is possible with resources r and b
fn is_possible(k: u64, r: u64, b: u64) -> bool {
    // We need to find non-negative integers p and q such that:
    // The total resources used does not exceed r and b
    // The calculations are based on the derived relationships
    for p in 0..=k {
        let q = k - p; // q is derived from k
        let total_resources_used = calcul(p, q);
        if total_resources_used <= r + b {
            return true; // Found a feasible allocation
        }
    }
    false // No feasible allocation found
}

// Calculate the total resources used based on p and q
fn calcul(p: u64, q: u64) -> u64 {
    // The calculation logic based on the relationships defined in the specification
    // This is a placeholder for the actual calculation logic
    // Assuming a simple linear relationship for demonstration
    p * p + q * q // Example: total resources used could be quadratic in nature
}

fn main() {
    let r: u64 = 10; // Example resource r
    let b: u64 = 5;  // Example resource b
    let result = max_allocation(r, b);
    println!("{}", result); // Output the result
}
