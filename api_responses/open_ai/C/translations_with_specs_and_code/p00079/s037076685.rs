use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn input_point() -> Option<Point> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                    return Some(Point { x, y });
                }
            }
        }
    }
    None
}

fn len(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).hypot(a.y - b.y))
}

fn area(x: &Point, y: &Point, z: &Point) -> f64 {
    let a = len(x, y);
    let b = len(y, z);
    let c = len(z, x);
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    let mut points = Vec::new();
    
    // Read points until no more valid points can be read
    while let Some(point) = input_point() {
        points.push(point);
    }

    // Ensure at least three points are provided
    if points.len() < 3 {
        eprintln!("Error: At least three points are required to form a polygon.");
        return;
    }

    // Calculate the area of the polygon
    let mut total_area = 0.0;
    for i in 2..points.len() {
        total_area += area(&points[0], &points[i - 1], &points[i]);
    }

    // Output the total area
    println!("{:.6}", total_area);
}
