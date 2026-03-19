use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of points
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("N must be a positive integer"),
        _ => {
            eprintln!("Error: Invalid input for N");
            std::process::exit(1);
        }
    };

    // Read boundary limits A, B, C, D
    let boundaries: Vec<i32> = match lines.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Boundary values must be integers"))
            .collect(),
        _ => {
            eprintln!("Error: Invalid input for boundaries");
            std::process::exit(1);
        }
    };

    if boundaries.len() != 4 {
        eprintln!("Error: Must provide exactly four boundary integers");
        std::process::exit(1);
    }

    let (a, b, c, d) = (boundaries[0], boundaries[1], boundaries[2], boundaries[3]);

    // Read coordinates
    let mut coordinates = Vec::new();
    for _ in 0..=n {
        match lines.next() {
            Some(Ok(line)) => {
                let coord: Vec<i32> = line
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().expect("Coordinate values must be integers"))
                    .collect();
                if coord.len() != 2 {
                    eprintln!("Error: Each coordinate must consist of exactly two integers");
                    std::process::exit(1);
                }
                coordinates.push((coord[0], coord[1]));
            }
            _ => {
                eprintln!("Error: Not enough coordinate inputs");
                std::process::exit(1);
            }
        }
    }

    // Calculate total cost based on movement between points
    let mut total_cost = 0;

    for i in 0..n {
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[i + 1];

        // Calculate the cost of moving from (x1, y1) to (x2, y2)
        let cost = ((x2 - x1).abs() + (y2 - y1).abs()) as i32;

        // Apply boundary constraints
        if x1 < a || x1 > b || y1 < c || y1 > d || x2 < a || x2 > b || y2 < c || y2 > d {
            // If out of bounds, apply a penalty (for example, double the cost)
            total_cost += cost * 2;
        } else {
            total_cost += cost;
        }
    }

    // Output the total cost
    println!("{}", total_cost);
}
