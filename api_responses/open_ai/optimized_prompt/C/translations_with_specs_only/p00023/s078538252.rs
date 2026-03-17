fn main() {
    use std::io::{self, BufRead};

    // Read the number of circle pairs
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Iterate over each pair of circles
    for _ in 0..n {
        // Read the circle parameters: x1, y1, r1, x2, y2, r2
        let params: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, r1, x2, y2, r2) = (params[0], params[1], params[2], params[3], params[4], params[5]);

        // Calculate the distance squared between the centers
        let dx = x1 - x2;
        let dy = y1 - y2;
        let distance_squared = dx * dx + dy * dy;

        // Calculate the radii squared
        let r1_squared = r1 * r1;
        let r2_squared = r2 * r2;

        // Determine the relationship between the circles
        let relationship = if distance_squared > (r1 + r2) * (r1 + r2) {
            0 // Circles do not intersect
        } else if distance_squared < (r1 - r2) * (r1 - r2) {
            if r1 > r2 {
                -2 // Circle 2 is completely within Circle 1
            } else {
                2 // Circle 1 is completely within Circle 2
            }
        } else {
            1 // Circles intersect
        };

        // Output the relationship
        println!("{}", relationship);
    }
}
