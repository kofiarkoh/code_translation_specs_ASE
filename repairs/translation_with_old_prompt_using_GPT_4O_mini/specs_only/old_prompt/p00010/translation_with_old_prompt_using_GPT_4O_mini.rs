use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap(); // Read number of triangles

    for _ in 0..n {
        let mut coords = String::new();
        io::stdin().read_line(&mut coords).unwrap();
        let values: Vec<f64> = coords
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3) = (values[0], values[1], values[2], values[3], values[4], values[5]);

        // Calculate the circumcenter and circumradius
        let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
        // The circumcenter coordinates (px, py)
        let px = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) + (x2.powi(2) + y2.powi(2)) * (y3 - y1) + (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
        let py = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) + (x2.powi(2) + y2.powi(2)) * (x1 - x3) + (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

        // Calculate the circumradius
        let r = ((x1 - px).powi(2) + (y1 - py).powi(2)).sqrt();

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", px, py, r);
    }
}
