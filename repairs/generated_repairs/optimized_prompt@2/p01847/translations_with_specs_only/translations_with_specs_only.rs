use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut tokens = line.split_whitespace();

        // Read the number of vertices
        let num_vertices: usize = match tokens.next() {
            Some(num) => num.parse().expect("Invalid number of vertices"),
            None => continue,
        };

        // Ensure the number of vertices is positive
        if num_vertices == 0 {
            continue; // No valid output for zero vertices
        }

        // Read polygon vertices
        let mut polygon: Vec<(i64, i64)> = Vec::new();
        for _ in 0..num_vertices {
            let x: i64 = match tokens.next() {
                Some(coord) => coord.parse().expect("Invalid x coordinate"),
                None => {
                    eprintln!("Missing x coordinate");
                    return;
                }
            };
            let y: i64 = match tokens.next() {
                Some(coord) => coord.parse().expect("Invalid y coordinate"),
                None => {
                    eprintln!("Missing y coordinate");
                    return;
                }
            };
            polygon.push((x, y));
        }

        // Read rectangle corners
        let mut rectangle: Vec<(i64, i64)> = Vec::new();
        for _ in 0..4 {
            let x: i64 = match tokens.next() {
                Some(coord) => coord.parse().expect("Invalid rectangle x coordinate"),
                None => {
                    eprintln!("Missing rectangle x coordinate");
                    return;
                }
            };
            let y: i64 = match tokens.next() {
                Some(coord) => coord.parse().expect("Invalid rectangle y coordinate"),
                None => {
                    eprintln!("Missing rectangle y coordinate");
                    return;
                }
            };
            rectangle.push((x, y));
        }

        // Calculate the area of the polygon
        let polygon_area = calculate_polygon_area(&polygon);
        // Calculate the area of the rectangle
        let rectangle_area = calculate_rectangle_area(&rectangle);
        // Calculate the area difference
        let area_difference = polygon_area - rectangle_area;

        // Print the area difference
        println!("{}", area_difference);
    }
}

// Function to calculate the area of a polygon using the shoelace formula
fn calculate_polygon_area(polygon: &[(i64, i64)]) -> i64 {
    let n = polygon.len();
    let mut area = 0;

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n]; // Wrap around to the first vertex
        area += x1 * y2 - x2 * y1;
    }

    (area.abs() / 2) // Return the absolute value of the area divided by 2
}

// Function to calculate the area of a rectangle given its corner points
fn calculate_rectangle_area(rectangle: &[(i64, i64)]) -> i64 {
    if rectangle.len() < 4 {
        eprintln!("Not enough rectangle corners provided");
        return 0;
    }
    let (x1, y1) = rectangle[0];
    let (x2, y2) = rectangle[2]; // Assuming rectangle is defined by two opposite corners
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    width * height // Return the area of the rectangle
}
