fn compute_count(n: i64, x: i64, d: i64) -> i64 {
    // Handle special cases based on the specification
    if x == 0 && d == 0 {
        return 1; // Both X and D are zero
    }
    if d == 0 {
        return n + 1; // D is zero
    }
    if x == 0 {
        return n * (n - 1) / 2 + 1; // X is zero and D is non-zero
    }

    // Initialize L, R, and V based on the transformations applied to X and D
    let l = x; // Assuming L is derived directly from X
    let r = x + d * n; // Assuming R is derived from X and D
    let v = (r - l) / d + 1; // Assuming V is the count of steps from L to R

    // Ensure the output is non-negative as per postconditions
    if v < 0 {
        return 0; // In case of any unexpected negative count
    }

    v // Return the computed count
}

fn main() {
    // Example usage
    let n: i64 = 10; // Example input
    let x: i64 = 2;  // Example input
    let d: i64 = 3;  // Example input
    let result = compute_count(n, x, d);
    println!("{}", result); // Output the result
}
