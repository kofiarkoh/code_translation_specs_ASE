use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read the number of triangle vertex sets
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    for _ in 0..n {
        input.clear();
        // Read six floating-point numbers for the triangle vertices
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid floating-point number"))
            .collect();

        if coords.len() != 6 {
            // If the number of coordinates is not 6, we have undefined behavior
            continue; // In a real scenario, we might want to handle this more gracefully
        }

        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        // Calculate the circumcenter and circumradius
        let (circumcenter_x, circumcenter_y, circumradius) = calculate_circumcircle(x1, y1, x2, y2, x3, y3);

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", circumcenter_x, circumcenter_y, circumradius);
    }
}

// Function to calculate the circumcenter and circumradius of a triangle
fn calculate_circumcircle(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> (f64, f64, f64) {
    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    
    // Check for collinearity (d should not be zero)
    if d.abs() < f64::EPSILON {
        // Collinear points, circumcenter is undefined; return NaN for circumradius
        return (f64::NAN, f64::NAN, f64::NAN);
    }

    let ux = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) + (x2.powi(2) + y2.powi(2)) * (y3 - y1) + (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
    let uy = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) + (x2.powi(2) + y2.powi(2)) * (x1 - x3) + (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

    // Calculate circumradius
    let circumradius = ((ux - x1).powi(2) + (uy - y1).powi(2)).sqrt();

    (ux, uy, circumradius)
}
