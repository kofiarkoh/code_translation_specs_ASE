fn main() {
    // Define the input ranges A, B and C, D
    let (a, b, c, d) = (10, 20, 15, 25); // Example input values

    // Ensure the preconditions are met (0 to 100, A <= B, C <= D)
    // In a real application, we would validate inputs, but the spec states
    // that behavior is undefined if preconditions are not met.

    // Initialize the overlap count
    let mut overlap_count = 0;

    // Calculate the number of overlapping indices
    for i in a..=b {
        if i >= c && i <= d {
            overlap_count += 1; // Increment count for each overlap
        }
    }

    // Adjust the count if there is at least one overlap
    if overlap_count > 0 {
        overlap_count -= 1; // Reduce by one if there is at least one overlap
    }

    // Output the result
    println!("{}", overlap_count);
}
