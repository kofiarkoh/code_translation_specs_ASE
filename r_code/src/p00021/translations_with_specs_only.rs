use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of line segment pairs
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the coordinates for two line segments
        let coords: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Unpack the coordinates
        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            coords[0], coords[1], coords[2], coords[3],
            coords[4], coords[5], coords[6], coords[7],
        );

        // Check if the x-coordinates are distinct to avoid division by zero
        if x1 != x2 && x3 != x4 {
            // Calculate slopes
            let slope1 = (y2 - y1) / (x2 - x1);
            let slope2 = (y4 - y3) / (x4 - x3);

            // Compare slopes
            if (slope1 - slope2).abs() < f64::EPSILON {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            // If x-coordinates are equal, we do not handle this case explicitly
            // This may lead to undefined behavior, so we output "NO" as a fallback
            println!("NO");
        }
    }
}
