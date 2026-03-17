fn count_overlaps(a: i32, b: i32, c: i32, d: i32) -> i32 {
    // Preconditions guarantee that 0 <= A <= B <= 100 and 0 <= C <= D <= 100
    // Calculate the start and end of the overlapping range
    let start = a.max(c);
    let end = b.min(d);
    
    // Calculate the number of overlapping indices
    let overlap_count = if start <= end {
        end - start + 1 // inclusive count of overlapping indices
    } else {
        0 // no overlap
    };
    
    // Adjust the count by subtracting one if there is at least one overlap
    if overlap_count > 0 {
        overlap_count - 1
    } else {
        0 // return 0 if there are no overlaps
    }
}

fn main() {
    // Example usage
    let a = 10;
    let b = 20;
    let c = 15;
    let d = 25;
    
    let result = count_overlaps(a, b, c, d);
    println!("{}", result); // Expected output: 4 (overlaps are 15, 16, 17, 18, 19, 20)
}
