use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of vertices
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Initialize vectors to store the polygon vertices
    let mut vertices = Vec::with_capacity(n);
    
    // Read the polygon vertices
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // Ensure we have exactly 2 coordinates for each vertex
        if coords.len() != 2 {
            panic!("Each vertex must have exactly 2 coordinates.");
        }
        vertices.push((coords[0], coords[1]));
    }

    // Read the rectangle coordinates
    let rect_coords: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have exactly 4 coordinates for the rectangle
    if rect_coords.len() != 4 {
        panic!("Rectangle must be defined by exactly 4 coordinates.");
    }
    
    let (rect_x1, rect_y1, rect_x2, rect_y2) = (rect_coords[0], rect_coords[1], rect_coords[2], rect_coords[3]);

    // Calculate the area of the polygon using the shoelace formula
    let mut area: i64 = 0;
    for i in 0..n {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % n]; // Wrap around to the first vertex
        area += (x0 as i64) * (y1 as i64) - (x1 as i64) * (y0 as i64);
    }
    area = (area.abs() + 1) / 2; // Area must be positive

    // Output the computed area
    println!("{}", area);
}
