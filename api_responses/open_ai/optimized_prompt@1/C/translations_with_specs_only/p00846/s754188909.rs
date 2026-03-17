// Define a struct to represent a point in 2D space
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Function to determine if a polygon intersects with a predefined convex shape
fn intersects(polygon: &[Point]) -> bool {
    // Placeholder for intersection logic with a predefined convex shape
    // This function should implement the actual intersection algorithm
    // For now, we will return false as a stub
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
