// This program calculates the area of a polygon defined by a set of vertices.
// The vertices are provided as input in the form of pairs of integers representing coordinates.

use std::io::{self, Write};

fn main() {
    // Read the number of vertices
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Preconditions: The number of vertices must be a positive integer
    if n == 0 {
        panic!("Number of vertices must be positive");
    }

    let mut vertices = Vec::new();

    // Read the vertex coordinates
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Coordinates must be integers"))
            .collect();

        // Each vertex must have exactly two coordinates
        if coords.len() != 2 {
            panic!("Each vertex must have exactly two coordinates");
        }

        vertices.push((coords[0], coords[1]));
    }

    // Calculate the area using the shoelace formula
    let area = calculate_area(&vertices);

    // Output the area formatted to one decimal place
    println!("{:.1}", area);
}

// Function to calculate the area of the polygon using the shoelace formula
fn calculate_area(vertices: &[(i32, i32)]) -> f64 {
    let mut area = 0;
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n]; // Wrap around to the first vertex
        area += x1 * y2 - x2 * y1;
    }

    // The area is non-negative, as per the postconditions
    (area.abs() as f64) / 2.0
}
