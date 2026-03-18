fn count_overlaps(a: i32, b: i32, c: i32, d: i32) -> i32 {
    // Preconditions: A <= B and C <= D, and all values are within 0 to 100 inclusive.
    // We assume these preconditions are met as per the specification.

    // Calculate the overlapping range
    let start = a.max(c); // The maximum of the starting points
    let end = b.min(d);   // The minimum of the ending points

    // Calculate the count of overlapping indices
    let overlap_count = if start <= end {
        end - start + 1 // Count of overlapping indices
    } else {
        0 // No overlap
    };

    // Adjust the count by reducing it by one if there is at least one overlap
    if overlap_count > 0 {
        return overlap_count - 1; // Return adjusted count
    }

    0 // Return 0 if no overlap
}

fn main() {
    // Example usage
    let a = 5;
    let b = 10;
    let c = 8;
    let d = 12;
    let result = count_overlaps(a, b, c, d);
    println!("{}", result); // Outputs the count of overlaps adjusted by the specification
}
