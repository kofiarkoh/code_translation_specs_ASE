use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read the number of sets of points
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_sets: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..num_sets {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        // Ensure we have exactly 6 coordinates for 3 points
        if coords.len() != 6 {
            eprintln!("Expected 6 coordinates for 3 points.");
            continue; // Skip this set if invalid
        }

        let (x1, y1) = (coords[0], coords[1]);
        let (x2, y2) = (coords[2], coords[3]);
        let (x3, y3) = (coords[4], coords[5]);

        // Calculate the circumcenter and radius
        let (circumcenter_x, circumcenter_y, radius) = calculate_circumcircle(x1, y1, x2, y2, x3, y3);

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3} {:.3}", circumcenter_x, circumcenter_y, radius);
    }
}

fn calculate_circumcircle(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> (f64, f64, f64) {
    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    
    // Check for collinearity (d should not be zero)
    if d.abs() < f64::EPSILON {
        // Collinear points; behavior is not defined
        return (0.0, 0.0, 0.0); // Placeholder values; undefined behavior
    }

    let ux = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) + (x2.powi(2) + y2.powi(2)) * (y3 - y1) + (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
    let uy = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) + (x2.powi(2) + y2.powi(2)) * (x1 - x3) + (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

    let radius = ((ux - x1).powi(2) + (uy - y1).powi(2)).sqrt(); // Distance from circumcenter to any point

    (ux, uy, radius)
}
