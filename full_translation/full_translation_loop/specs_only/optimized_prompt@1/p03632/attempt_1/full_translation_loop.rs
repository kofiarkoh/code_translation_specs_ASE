fn count_overlaps(a: i32, b: i32, c: i32, d: i32) -> i32 {
    // Preconditions: A <= B and C <= D, and all values are within 0 to 100 inclusive.
    // We assume these preconditions are met as per the specification.

    // Initialize the count of overlaps
    let mut overlap_count = 0;

    // Calculate the overlapping range
    let start = a.max(c); // The maximum of the starting points
    let end = b.min(d);   // The minimum of the ending points

    // If there is an overlap, calculate the number of overlapping indices
    if start <= end {
        overlap_count = end - start + 1; // Count of overlapping indices
    }

    // Adjust the count by reducing it by one if there is at least one overlap
    if overlap_count > 0 {
        overlap_count -= 1;
    }

    // Ensure the output is non-negative
    overlap_count.max(0) // Return non-negative output
}

fn main() {
    // Example usage
    let a = 10;
    let b = 20;
    let c = 15;
    let d = 25;
    let result = count_overlaps(a, b, c, d);
    println!("{}", result); // Outputs the count of overlaps adjusted by the specification
}
