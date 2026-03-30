fn count_overlapping_integers(a: i32, b: i32, c: i32, d: i32) -> i32 {
    // Preconditions guarantee that 0 <= A, B, C, D <= 100 and A <= B, C <= D
    // Initialize a count for overlapping integers
    let mut overlap_count = 0;

    // Calculate the start and end of the overlapping range
    let start = a.max(c);
    let end = b.min(d);

    // If there is an overlap, count the integers in the overlapping range
    if start <= end {
        overlap_count = end - start + 1; // Count inclusive
    }

    // Adjust the count by subtracting one if there is at least one overlap
    if overlap_count > 0 {
        overlap_count -= 1;
    }

    overlap_count
}

// Example usage (not part of the specification, just for testing):
fn main() {
    let a = 1;
    let b = 5;
    let c = 4;
    let d = 8;
    let result = count_overlapping_integers(a, b, c, d);
    println!("{}", result); // Expected output: 0 (since overlap is 4, 5)
}
