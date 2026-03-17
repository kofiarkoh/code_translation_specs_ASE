fn main() {
    // Example input: (x1, y1, r1, x2, y2, r2)
    let (x1, y1, r1, x2, y2, r2) = (0, 0, 5, 10, 10, 5); // Replace with actual input

    let relationship = determine_circle_relationship(x1, y1, r1, x2, y2, r2);
    println!("{}", relationship);
}

// Determines the spatial relationship between two circles
fn determine_circle_relationship(x1: i32, y1: i32, r1: i32, x2: i32, y2: i32, r2: i32) -> i32 {
    // Calculate the squared distance between the centers to avoid floating-point operations
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance_squared = (dx * dx + dy * dy) as u32; // Use u32 to prevent negative values

    // Calculate the squared radii
    let r1_squared = (r1 * r1) as u32;
    let r2_squared = (r2 * r2) as u32;

    // Determine the relationship based on the distance and radii
    if distance_squared > (r1 + r2) as u32 * (r1 + r2) as u32 {
        // Circles are separate
        return 0;
    } else if distance_squared < (r1 - r2).abs() as u32 * (r1 - r2).abs() as u32 {
        // One circle is completely inside the other
        return 1;
    } else if distance_squared == (r1 + r2) as u32 * (r1 + r2) as u32 {
        // Circles are touching externally
        return 2;
    } else if distance_squared == (r1 - r2).abs() as u32 * (r1 - r2).abs() as u32 {
        // Circles are touching internally
        return 3;
    } else {
        // Circles are overlapping
        return 4;
    }
}
