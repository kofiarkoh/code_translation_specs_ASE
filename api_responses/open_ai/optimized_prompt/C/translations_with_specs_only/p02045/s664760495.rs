use std::io::{self, Write};

fn main() {
    // Read the number of points
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            // Terminate with return value 1 if N is non-positive
            std::process::exit(1);
        }
    };

    // Read boundary limits A, B, C, D
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let boundaries: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if boundaries.len() != 4 {
        // Terminate with return value 1 if boundary inputs are invalid
        std::process::exit(1);
    }

    let (a, b, c, d) = (boundaries[0], boundaries[1], boundaries[2], boundaries[3]);

    // Read coordinates
    let mut coordinates = Vec::new();
    for _ in 0..=n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let point: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if point.len() != 2 {
            // Terminate with return value 1 if coordinates are invalid
            std::process::exit(1);
        }

        coordinates.push((point[0], point[1]));
    }

    // Calculate total cost
    let mut total_cost = 0;

    for i in 0..n {
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[i + 1];

        // Calculate movement cost
        let movement_cost = ((x2 - x1).abs() + (y2 - y1).abs()) as i32;

        // Check for boundary violations and apply penalties
        let penalty = if x2 < a || x2 > b || y2 < c || y2 > d {
            10 // Example penalty for moving outside boundaries
        } else {
            0
        };

        total_cost += movement_cost + penalty;
    }

    // Output the total cost
    println!("{}", total_cost);
}
