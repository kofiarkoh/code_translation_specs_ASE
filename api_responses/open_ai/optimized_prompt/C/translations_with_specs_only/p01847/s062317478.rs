use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut tokens = line.split_whitespace();

        // Read the number of vertices
        let num_vertices: i32 = match tokens.next() {
            Some(token) => token.parse().expect("Invalid integer input"),
            None => continue,
        };

        // Terminate if the number of vertices is non-positive
        if num_vertices <= 0 {
            break;
        }

        // Read the vertices
        let mut vertices = Vec::new();
        for _ in 0..num_vertices {
            let x: i32 = tokens.next().expect("Expected x coordinate").parse().expect("Invalid integer input");
            let y: i32 = tokens.next().expect("Expected y coordinate").parse().expect("Invalid integer input");
            vertices.push((x, y));
        }

        // Read the bounding box coordinates
        let mut bounding_box = Vec::new();
        for _ in 0..4 {
            let x: i32 = tokens.next().expect("Expected x coordinate").parse().expect("Invalid integer input");
            let y: i32 = tokens.next().expect("Expected y coordinate").parse().expect("Invalid integer input");
            bounding_box.push((x, y));
        }

        // Calculate the area difference
        let polygon_area = calculate_polygon_area(&vertices);
        let bounding_box_area = calculate_bounding_box_area(&bounding_box);
        let area_difference = polygon_area - bounding_box_area;

        // Output the area difference
        println!("{}", area_difference);
    }
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
    (area.abs() / 2) // Return the absolute value of the area divided by 2
}

// Function to calculate the area of a bounding box
fn calculate_bounding_box_area(bounding_box: &[(i32, i32)]) -> i64 {
    let (x1, y1) = bounding_box[0];
    let (x2, y2) = bounding_box[2]; // Assuming the bounding box is defined by two opposite corners
    let width = (x2 - x1).abs() as i64;
    let height = (y2 - y1).abs() as i64;
    width * height // Area of the rectangle
}
