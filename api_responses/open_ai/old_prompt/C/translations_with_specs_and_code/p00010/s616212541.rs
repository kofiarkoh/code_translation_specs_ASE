use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of triangles
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the coordinates of the triangle vertices
        let coords: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        let a1 = 2.0 * (x2 - x1);
        let b1 = 2.0 * (y2 - y1);
        let c1 = x1 * x1 - x2 * x2 + y1 * y1 - y2 * y2;

        let a2 = 2.0 * (x3 - x1);
        let b2 = 2.0 * (y3 - y1);
        let c2 = x1 * x1 - x3 * x3 + y1 * y1 - y3 * y3;

        // Calculate the circumcenter (px, py)
        let denominator = a1 * b2 - a2 * b1;
        let px = (b1 * c2 - b2 * c1) / denominator;
        let py = (c1 * a2 - c2 * a1) / denominator;

        // Calculate the circumradius (r)
        let s = (x1 - px).powi(2) + (y1 - py).powi(2);
        let mut r = s / 2.0;

        // Use Newton's method to approximate the radius
        let mut x = 0.0;
        while (r - x).abs() > f64::EPSILON {
            x = r;
            r = (r + s / r) / 2.0;
        }

        // Ensure the radius is non-negative
        if r < 0.0 {
            r *= -1.0;
        }

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", px, py, r);
    }
}
