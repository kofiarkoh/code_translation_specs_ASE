use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of vertices
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for number of vertices");
    
    // Preconditions: n must be a positive integer
    if n == 0 {
        panic!("The number of vertices must be a positive integer.");
    }

    let mut points = Vec::with_capacity(n);
    
    // Read the vertex coordinates
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for vertex coordinates"))
            .collect();
        points.push((coords[0], coords[1]));
    }

    let mut area = 0.0;

    // Calculate the area using the shoelace formula
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n]; // Wrap around to the first point
        area += (x1 - x2) as f64 * (y1 + y2) as f64;
    }

    // Output the area formatted to one decimal place
    println!("{:.1}", area / 2.0);
}
