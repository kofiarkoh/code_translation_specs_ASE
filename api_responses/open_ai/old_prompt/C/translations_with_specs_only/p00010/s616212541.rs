use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of triangles
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the coordinates of the triangle vertices
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        // Calculate the circumcenter and circumradius
        let (px, py, r) = calculate_circumcenter_and_radius(x1, y1, x2, y2, x3, y3);

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", px, py, r);
    }
}

// Function to calculate the circumcenter and circumradius of a triangle
fn calculate_circumcenter_and_radius(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> (f64, f64, f64) {
    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    
    // The circumcenter coordinates (px, py)
    let px = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) + (x2.powi(2) + y2.powi(2)) * (y3 - y1) + (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
    let py = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) + (x2.powi(2) + y2.powi(2)) * (x1 - x3) + (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

    // The circumradius (r)
    let r = ((x1 - px).hypot(y1 - py) + (x2 - px).hypot(y2 - py) + (x3 - px).hypot(y3 - py)) / 3.0;

    (px, py, r)
}
