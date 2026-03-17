fn main() {
    // Example input: number of circle pairs and their properties
    let num_pairs: usize = 2; // This should be read from input in a real scenario
    let circle_data: Vec<(f64, f64, f64, f64, f64, f64)> = vec![
        (0.0, 0.0, 1.0, 1.0, 1.0, 1.0), // Example circles
        (0.0, 0.0, 1.0, 2.0, 2.0, 0.5),
    ];

    for (x1, y1, r1, x2, y2, r2) in circle_data.iter().take(num_pairs) {
        let result = classify_circles(*x1, *y1, *r1, *x2, *y2, *r2);
        println!("{}", result);
    }
}

// Classifies the relationship between two circles based on their centers and radii
fn classify_circles(x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64) -> i32 {
    // Calculate the distance squared between the centers to avoid floating-point inaccuracies
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance_squared = dx * dx + dy * dy;

    // Calculate the radii squared for comparison
    let r1_squared = r1 * r1;
    let r2_squared = r2 * r2;

    // Check for intersection and containment
    if distance_squared > (r1 + r2) * (r1 + r2) {
        // Circles do not intersect
        return 0;
    } else if distance_squared < (r1 - r2) * (r1 - r2) {
        // One circle is completely within the other
        if r1 > r2 {
            return 2; // First circle is larger
        } else {
            return -2; // Second circle is larger
        }
    } else {
        // Circles intersect
        return 1;
    }
}
