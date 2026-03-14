use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if values.len() != 8 {
            continue; // Skip invalid input sets
        }

        let (x1, y1, x2, y2, x3, y3, px, py) = (
            values[0], values[1], values[2], values[3],
            values[4], values[5], values[6], values[7]
        );

        if is_point_in_triangle(px, py, x1, y1, x2, y2, x3, y3) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn is_point_in_triangle(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> bool {
    let area = triangle_area(x1, y1, x2, y2, x3, y3);
    let area1 = triangle_area(px, py, x2, y2, x3, y3);
    let area2 = triangle_area(x1, y1, px, py, x3, y3);
    let area3 = triangle_area(x1, y1, x2, y2, px, py);

    // Use a small tolerance to compare floating-point numbers
    let tolerance = 1e-8;
    (area - (area1 + area2 + area3)).abs() < tolerance
}

fn triangle_area(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
    (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0
}
