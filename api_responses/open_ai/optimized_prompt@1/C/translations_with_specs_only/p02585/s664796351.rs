fn compute_max_value(n: usize, k: usize, p: &[i32], c: &[i32]) -> i64 {
    // Preconditions check
    assert!(n > 0, "N must be a positive integer.");
    assert!(k >= 0, "K must be a non-negative integer.");
    assert!(p.len() == n, "Length of P must be equal to N.");
    assert!(c.len() == n, "Length of C must be equal to N.");

    let mut max_value: i64 = i64::MIN; // Initialize to the minimum possible value
    let mut operations = 0;

    // Iterate through the positions and costs
    for i in 0..n {
        // Check if the position is a valid index
        if p[i] >= 0 && (p[i] as usize) < n {
            // Perform the operation if we have not exceeded the limit
            if operations < k {
                max_value = max_value.max(c[i] as i64); // Update max_value based on costs
                operations += 1; // Increment the number of operations performed
            }
        }
    }

    // If no valid operations were performed, return a specific minimum value
    if operations == 0 {
        return -1; // Assuming -1 is the specific minimum value when no operations can be performed
    }

    max_value // Return the computed maximum value
}

fn main() {
    // Example usage
    let n = 5;
    let k = 3;
    let p = [0, 1, 2, 3, 4];
    let c = [10, 20, 30, 40, 50];

    let result = compute_max_value(n, k, &p, &c);
    println!("Maximum computed value: {}", result);
}
