use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of vertices
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Ensure that we have at least one vertex
    if n == 0 {
        return; // No output for zero vertices
    }

    // Read the polygon vertices
    let mut vertices = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vertices.push((coords[0], coords[1]));
    }

    // Read the rectangle coordinates
    let rect_coords: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (x1, y1, x2, y2) = (rect_coords[0], rect_coords[1], rect_coords[2], rect_coords[3]);

    // Calculate the area of the polygon
    let area = calculate_polygon_area(&vertices);
    
    // Adjust the area based on the intersection with the rectangle
    let adjusted_area = adjust_area_with_rectangle(area, &vertices, (x1, y1), (x2, y2));

    // Output the adjusted area
    println!("{}", adjusted_area);
}

// Function to calculate the area of a polygon using the shoelace formula
fn calculate_polygon_area(vertices: &[(i32, i32)]) -> i64 {
    let mut area = 0;
    let n = vertices.len();
    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n]; // Wrap around to the first vertex
        area += x1 as i64 * y2 as i64 - x2 as i64 * y1 as i64;
    }
    (area.abs() + 1) / 2 // Return the absolute value of the area divided by 2
}

// Function to adjust the area based on the intersection with the rectangle
fn adjust_area_with_rectangle(area: i64, vertices: &[(i32, i32)], rect_bottom_left: (i32, i32), rect_top_right: (i32, i32)) -> i64 {
    // Placeholder for actual intersection area calculation
    // This function should compute the area of the polygon that lies within the rectangle
    // For now, we will return the original area as a stub
    area // In a complete implementation, this would subtract the area outside the rectangle
}
