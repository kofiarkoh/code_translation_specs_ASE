use std::io::{self, Write};

fn main() {
    // Read the number of vertices
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before input
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for number of vertices");

    // Ensure n is greater than 0
    if n == 0 {
        panic!("Number of vertices must be greater than 0");
    }

    // Initialize a vector to store the vertices
    let mut vertices = Vec::with_capacity(n);

    // Read the coordinates of the vertices
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate input"))
            .collect();
        
        // Ensure we have exactly two coordinates
        if coords.len() != 2 {
            panic!("Each vertex must have exactly two coordinates");
        }
        
        vertices.push((coords[0], coords[1]));
    }

    // Calculate the area using the shoelace formula
    let mut area: i64 = 0; // Use i64 to prevent overflow during calculations
    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n]; // Wrap around to the first vertex
        area += (x1 as i64) * (y2 as i64) - (x2 as i64) * (y1 as i64);
    }

    // The area is the absolute value divided by 2
    let area = (area.abs() as f64) / 2.0;

    // Print the area formatted to one decimal place
    println!("{:.1}", area);
}
