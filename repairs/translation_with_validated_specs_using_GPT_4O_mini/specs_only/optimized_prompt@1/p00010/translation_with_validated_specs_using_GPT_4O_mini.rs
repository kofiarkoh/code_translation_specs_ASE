use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for number of triangles");

    for _ in 0..n {
        let mut triangle_input = String::new();
        io::stdin().read_line(&mut triangle_input).expect("Failed to read triangle vertices");
        let coords: Vec<f64> = triangle_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        // Unpack the coordinates
        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        // Calculate the circumcenter
        let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
        let ux = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) + (x2.powi(2) + y2.powi(2)) * (y3 - y1) + (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
        let uy = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) + (x2.powi(2) + y2.powi(2)) * (x1 - x3) + (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

        // Calculate the circumradius
        let circumradius = ((x1 - ux).powi(2) + (y1 - uy).powi(2)).sqrt();

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", ux, uy, circumradius);
    }
}
