// Define a struct to represent a point in 2D space
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Function to determine if a polygon intersects with a predefined convex shape
fn intersects(polygon: &[Point]) -> bool {
    // Implementing a simple check for intersection with a predefined convex shape
    // Here we assume the predefined shape is a rectangle defined by (0,0) to (20000,20000)
    let rect_min = Point { x: 0, y: 0 };
    let rect_max = Point { x: 20000, y: 20000 };

    // Check if any point of the polygon is inside the rectangle
    for point in polygon {
        if point.x >= rect_min.x && point.x <= rect_max.x &&
           point.y >= rect_min.y && point.y <= rect_max.y {
            return true;
        }
    }

    // Check if any edge of the polygon intersects with the rectangle
    for i in 0..polygon.len() {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % polygon.len()];

        // Check for intersection with rectangle edges
        if line_intersects_rect(p1, p2, &rect_min, &rect_max) {
            return true;
        }
    }

    false
}

// Function to check if a line segment intersects with the rectangle
fn line_intersects_rect(p1: &Point, p2: &Point, rect_min: &Point, rect_max: &Point) -> bool {
    // Check if the line segment intersects with any of the rectangle's edges
    // Implementing a basic line-rectangle intersection logic
    let edges = [
        (rect_min, Point { x: rect_max.x, y: rect_min.y }), // Bottom edge
        (Point { x: rect_max.x, y: rect_min.y }, rect_max), // Right edge
        (rect_max, Point { x: rect_min.x, y: rect_max.y }), // Top edge
        (Point { x: rect_min.x, y: rect_max.y }, rect_min), // Left edge
    ];

    for &(edge_start, edge_end) in &edges {
        if line_intersects(p1, p2, &edge_start, &edge_end) {
            return true;
        }
    }

    false
}

// Function to check if two line segments intersect
fn line_intersects(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let orientation = |p: &Point, q: &Point, r: &Point| {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 { 0 } else if val > 0 { 1 } else { 2 }
    };

    let o1 = orientation(p1, p2, p3);
    let o2 = orientation(p1, p2, p4);
    let o3 = orientation(p3, p4, p1);
    let o4 = orientation(p3, p4, p2);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    false
}

// Main function to read input and output the intersection result
fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line to get the number of vertices
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Create a vector to hold the polygon vertices
    let mut polygon = Vec::with_capacity(n);

    // Read the pairs of integers representing the coordinates of each vertex
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        polygon.push(Point { x: coords[0], y: coords[1] });
    }

    // Check for intersection and output the result
    if intersects(&polygon) {
        println!("1");
    } else {
        println!("0");
    }
}
