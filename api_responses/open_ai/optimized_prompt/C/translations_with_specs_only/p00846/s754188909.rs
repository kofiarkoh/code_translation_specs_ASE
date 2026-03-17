use std::io::{self, BufRead};

const EPSILON: f64 = 1e-9; // Define a small epsilon for floating-point comparisons

// Define a structure to represent a point in 2D space
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Function to check if two line segments intersect
fn do_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
    // Helper function to calculate the orientation of the ordered triplet (p, q, r)
    fn orientation(p: Point, q: Point, r: Point) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if (val > EPSILON) as i32 {
            1 // Clockwise
        } else if (val < -EPSILON) as i32 {
            2 // Counterclockwise
        } else {
            0 // Collinear
        }
    }

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases (collinear points)
    // Check if the points are on the segment
    fn on_segment(p: Point, q: Point, r: Point) -> bool {
        q.x <= r.x.max(p.x) && q.x >= r.x.min(p.x) && q.y <= r.y.max(p.y) && q.y >= r.y.min(p.y)
    }

    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    false
}

// Function to check if the polygon intersects with the convex shape
fn polygon_intersects(vertices: &[Point], convex_shape: &[Point]) -> bool {
    let n = vertices.len();
    let m = convex_shape.len();

    // Check each edge of the polygon against each edge of the convex shape
    for i in 0..n {
        let p1 = vertices[i];
        let q1 = vertices[(i + 1) % n]; // Wrap around to the first vertex

        for j in 0..m {
            let p2 = convex_shape[j];
            let q2 = convex_shape[(j + 1) % m]; // Wrap around to the first vertex

            if do_intersect(p1, q1, p2, q2) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of vertices
    let first_line = lines.next().unwrap().unwrap();
    let num_vertices: usize = first_line.trim().parse().unwrap();

    // Check precondition for number of vertices
    if num_vertices == 0 {
        return; // Terminate without output
    }

    let mut vertices = Vec::with_capacity(num_vertices + 1); // +1 for closing the polygon

    // Read the vertices
    for _ in 0..num_vertices {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vertices.push(Point { x: coords[0], y: coords[1] });
    }

    // Close the polygon by duplicating the first vertex
    vertices.push(vertices[0]);

    // Define the convex shape (example: a triangle)
    let convex_shape = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 5.0, y: 1.0 },
        Point { x: 3.0, y: 4.0 },
    ];

    // Check for intersection
    let intersects = polygon_intersects(&vertices, &convex_shape);

    // Output the result
    if intersects {
        println!("1");
    } else {
        println!("0");
    }
}
