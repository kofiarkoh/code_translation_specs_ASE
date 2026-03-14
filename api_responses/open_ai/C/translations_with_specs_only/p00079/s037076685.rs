use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let points: Vec<(f64, f64)> = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect();

    // Ensure at least three points are provided to form a polygon
    if points.len() < 3 {
        eprintln!("Error: At least three points are required to form a polygon.");
        return;
    }

    let area = calculate_polygon_area(&points);
    println!("{:.6}", area);
}

fn calculate_polygon_area(points: &[(f64, f64)]) -> f64 {
    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n]; // Wrap around to the first point
        area += x1 * y2 - x2 * y1;
    }

    (area.abs()) / 2.0 // Area must be non-negative
}
