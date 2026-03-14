use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly 6 values
        if values.len() != 6 {
            // Undefined behavior as per specification; we just continue to the next line
            continue;
        }

        // Unpack the input values
        let (x1, y1, x2, y2, px, py) = (values[0], values[1], values[2], values[3], values[4], values[5]);

        // Calculate the reflection point
        let (dx, dy) = (x2 - x1, y2 - y1);
        let (a, b) = (dy, -dx); // Perpendicular vector
        let (c, d) = (px - x1, py - y1); // Vector from point on line to point to reflect

        // Calculate the dot product and the projection length
        let dot_product = a * c + b * d;
        let length_squared = a * a + b * b;

        // Projection factor
        let projection = dot_product / length_squared;

        // Calculate the reflection point
        let rx = px - 2.0 * projection * a;
        let ry = py - 2.0 * projection * b;

        // Output the reflection point
        println!("{} {}", rx, ry);
    }
}
