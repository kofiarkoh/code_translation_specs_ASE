use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        
        // Read the number of vertices
        let num_vertices: usize = tokens.next().unwrap().parse().unwrap();
        let mut vertices = Vec::with_capacity(num_vertices);
        
        // Read the vertices
        for _ in 0..num_vertices {
            let x: i32 = tokens.next().unwrap().parse().unwrap();
            let y: i32 = tokens.next().unwrap().parse().unwrap();
            vertices.push((x, y));
        }

        // Read the bounding box coordinates
        let mut bounding_box = Vec::new();
        for _ in 0..4 {
            let x: i32 = tokens.next().unwrap().parse().unwrap();
            let y: i32 = tokens.next().unwrap().parse().unwrap();
            bounding_box.push((x, y));
        }

        // Calculate the area of the polygon using the shoelace formula
        let polygon_area = calculate_polygon_area(&vertices);
        
        // Calculate the area of the bounding box
        let bounding_box_area = calculate_bounding_box_area(&bounding_box);
        
        // Calculate the area difference
        let area_difference = polygon_area - bounding_box_area;

        // Output the area difference as a long integer
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
    // Calculate the min and max x and y coordinates from the bounding box
    let min_x = bounding_box.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = bounding_box.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = bounding_box.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = bounding_box.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x).abs() as i64;
    let height = (max_y - min_y).abs() as i64;
    width * height // Return the area of the bounding box
}
