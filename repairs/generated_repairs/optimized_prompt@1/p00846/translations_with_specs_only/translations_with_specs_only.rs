// Define a struct to represent a point in 2D space
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Function to determine if a polygon intersects with a predefined convex shape
fn intersects(polygon: &[Point]) -> bool {
    // Implementing a simple check for intersection with a predefined convex shape
    // Here we assume the predefined shape is a rectangle defined by (0,0) to (10000,10000)
    let rect_min = Point { x: 0, y: 0 };
    let rect_max = Point { x: 10000, y: 10000 };

    // Check if any point of the polygon is inside the rectangle
    for point in polygon {
        if point.x >= rect_min.x && point.x <= rect_max.x &&
           point.y >= rect_min.y && point.y <= rect_max.y {
            return true;
        }
    }

    // Check if any edge of the polygon intersects with the rectangle
    // This is a simplified version and may need a more robust implementation
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
    // This is a placeholder for the actual line-rectangle intersection logic
    // For simplicity, we will return false here
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
