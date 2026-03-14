fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of datasets
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the first circle's parameters
        let circle1: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        // Read the second circle's parameters
        let circle2: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Extract parameters
        let (x1, y1, r1) = (circle1[0], circle1[1], circle1[2]);
        let (x2, y2, r2) = (circle2[0], circle2[1], circle2[2]);

        // Calculate the distance squared between the centers
        let dx = x1 - x2;
        let dy = y1 - y2;
        let distance_squared = dx * dx + dy * dy;
        
        // Calculate the radii sum and difference
        let r_sum = r1 + r2;
        let r_diff = (r1 - r2).abs();

        // Determine the relationship
        let relationship = if distance_squared == 0.0 && r1 == r2 {
            2 // Circles are identical
        } else if distance_squared < r_diff * r_diff {
            -2 // One circle is contained within the other
        } else if distance_squared < r_sum * r_sum {
            1 // Circles intersect
        } else {
            0 // Circles are separate
        };

        // Output the relationship
        println!("{}", relationship);
    }
}
