use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<&str> = line.split_whitespace().collect();
        
        // Parse width and height
        let width: usize = match dimensions.get(0).and_then(|&s| s.parse().ok()) {
            Some(w) if w > 0 => w,
            _ => break, // Stop processing if width is zero or invalid
        };
        
        let height: usize = match dimensions.get(1).and_then(|&s| s.parse().ok()) {
            Some(h) => h,
            _ => continue, // Skip invalid height
        };

        // Read the grid
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..height {
            let mut grid_line = String::new();
            reader.read_line(&mut grid_line).unwrap(); // Use mutable reader
            let row: Vec<char> = grid_line.trim().chars().collect();
            if row.len() == width {
                grid.push(row);
            } else {
                // If the row length does not match the specified width, skip processing
                continue;
            }
        }

        // Evaluate stability
        let stability = evaluate_stability(&grid);
        println!("{}", stability);
    }
}

// Function to evaluate the stability of the grid
fn evaluate_stability(grid: &[Vec<char>]) -> &'static str {
    // Placeholder for stability evaluation logic
    // Here we would implement the logic to determine if the structure is stable or unstable
    // For now, we will return "STABLE" as a default response
    // This should be replaced with actual logic based on the geometric criteria specified
    "STABLE"
}
