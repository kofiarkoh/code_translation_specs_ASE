use std::io::{self, BufRead};

/// Classifies the area of a triangle based on its size.
fn classify_area(area: f64) -> &'static str {
    if area < 1.0 {
        "kyo"
    } else if area < 5.0 {
        "syo-kichi"
    } else if area < 10.0 {
        "kichi"
    } else if area < 20.0 {
        "chu-kichi"
    } else {
        "dai-kichi"
    }
}

/// Calculates the area of a triangle given by three points.
fn triangle_area(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    0.5 * ((ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)).abs())
}

/// Checks if two line segments intersect and calculates the intersection point if they do.
fn intersect_segments(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, dx: f64, dy: f64, x: &mut f64, y: &mut f64) -> i32 {
    let denom = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
    if denom == 0.0 {
        return 0; // Lines are parallel
    }

    let t = ((ax - cx) * (dy - cy) - (ay - cy) * (dx - cx)) / denom;
    let u = -((ax - bx) * (ay - cy) - (ay - by) * (ax - cx)) / denom;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        *x = ax + t * (bx - ax);
        *y = ay + t * (by - ay);
        return 1; // Intersection occurs
    }
    0 // No intersection
}

/// Main function to read input and process triangle area classification.
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if coords.len() != 8 {
            continue; // Skip invalid input
        }

        let (ax, ay, bx, by, cx, cy, dx, dy) = (
            coords[0] as f64, coords[1] as f64,
            coords[2] as f64, coords[3] as f64,
            coords[4] as f64, coords[5] as f64,
            coords[6] as f64, coords[7] as f64,
        );

        let mut intersection_x = 0.0;
        let mut intersection_y = 0.0;

        // Check for intersection between the two segments
        if intersect_segments(ax, ay, bx, by, cx, cy, dx, dy, &mut intersection_x, &mut intersection_y) == 1 {
            // Calculate area of triangle formed by the intersection point and the original points
            let area = triangle_area(ax, ay, bx, by, intersection_x, intersection_y);
            let classification = classify_area(area);
            println!("{}", classification);
        } else {
            println!("kyo"); // No intersection
        }
    }
}
