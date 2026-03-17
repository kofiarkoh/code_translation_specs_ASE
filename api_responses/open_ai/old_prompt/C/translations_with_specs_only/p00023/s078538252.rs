fn main() {
    // Read the number of circle pairs to evaluate
    let n: usize = read_input(); // Assume a function to read input as usize

    // If n is zero, we terminate without output
    if n == 0 {
        return;
    }

    for _ in 0..n {
        // Read properties of the first circle
        let (ax, ay, ar) = read_circle_properties(); // Assume a function to read circle properties
        // Read properties of the second circle
        let (bx, by, br) = read_circle_properties(); // Assume a function to read circle properties

        // Calculate the squared distances to avoid floating-point precision issues
        let dx = ax - bx;
        let dy = ay - by;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum_squared = (ar + br) * (ar + br);
        let radius_diff_squared = (ar - br).max(0.0) * (ar - br).max(0.0);

        // Determine the relationship between the circles
        let relationship_code = if distance_squared > radius_sum_squared {
            0 // Circles are separate
        } else if distance_squared < radius_diff_squared {
            if ar > br {
                2 // Circle A contains Circle B
            } else {
                -2 // Circle B contains Circle A
            }
        } else {
            1 // Circles intersect
        };

        // Output the relationship code
        println!("{}", relationship_code);
    }
}

// Placeholder function to read input as usize
fn read_input() -> usize {
    // Implementation to read an integer from standard input
    // This is a stub; actual implementation would handle input reading
    0 // Replace with actual input reading logic
}

// Placeholder function to read circle properties
fn read_circle_properties() -> (f64, f64, f64) {
    // Implementation to read circle properties (x, y, r) from standard input
    // This is a stub; actual implementation would handle input reading
    (0.0, 0.0, 0.0) // Replace with actual input reading logic
}
