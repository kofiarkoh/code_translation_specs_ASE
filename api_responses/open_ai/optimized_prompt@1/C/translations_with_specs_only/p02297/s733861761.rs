use std::io::{self, Write};

fn main() {
    // Read the number of vertices
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_vertices: usize = input.trim().parse().expect("Invalid number of vertices");

    // Preconditions: The number of vertices must be a positive integer
    if num_vertices == 0 {
        panic!("Number of vertices must be a positive integer");
    }

    let mut vertices = Vec::new();

    // Read the vertex coordinates
    for _ in 0..num_vertices {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        // Each vertex must have exactly two coordinates
        if coords.len() != 2 {
            panic!("Each vertex must have exactly two coordinates");
        }

        vertices.push((coords[0], coords[1]));
    }

    // Calculate the area using the Shoelace formula
    let area = calculate_area(&vertices);
    
    // Output the area formatted to one decimal place
    println!("{:.1}", area);
}

// Function to calculate the area of a polygon using the Shoelace formula
fn calculate_area(vertices: &[(i32, i32)]) -> f64 {
    let mut area = 0.0;
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n]; // Wrap around to the first vertex
        area += (x1 * y2) as f64 - (y1 * x2) as f64;
    }

    (area.abs() / 2.0).max(0.0) // Ensure area is non-negative
}
