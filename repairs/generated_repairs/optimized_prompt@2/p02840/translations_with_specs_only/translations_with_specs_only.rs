fn compute_count(n: i64, x: i64, d: i64) -> i64 {
    // Preconditions: N must be a non-negative integer.
    if n < 0 {
        panic!("N must be a non-negative integer");
    }

    // Handle exceptional behavior based on the values of X and D
    if x == 0 && d == 0 {
        return 1; // Both X and D are zero
    } else if d == 0 {
        return n + 1; // D is zero
    } else if x == 0 {
        return n * (n + 1) / 2; // X is zero and D is non-zero
    }

    // Calculate the count based on the values of X and D
    let mut count = 0;
    for i in 0..=n {
        count += (i / d) * x + x; // Adjust the logic to compute the count based on X and D
    }

    count
}

fn main() {
    // Example usage
    let n: i64 = 10; // Example input
    let x: i64 = 5;  // Example input
    let d: i64 = 3;  // Example input
    let result = compute_count(n, x, d);
    println!("{}", result); // Output the result
}
