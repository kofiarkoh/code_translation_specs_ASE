fn main() {
    // Read input values
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have enough inputs
    if values.len() < 5 {
        println!("1"); // Return 1 on invalid input
        return;
    }

    // Extract parameters
    let n = values.remove(0); // Number of points
    let a = values.remove(0); // Boundary A
    let b = values.remove(0); // Boundary B
    let c = values.remove(0); // Boundary C
    let d = values.remove(0); // Boundary D

    // Check precondition for N
    if n <= 0 {
        println!("1"); // Return 1 if N is 0 or negative
        return;
    }

    // Collect coordinates
    let mut x_coords = Vec::new();
    let mut y_coords = Vec::new();

    for i in 0..n {
        if let Some(x) = values.get((i * 2) as usize) {
            x_coords.push(*x);
        }
        if let Some(y) = values.get((i * 2 + 1) as usize) {
            y_coords.push(*y);
        }
    }

    // Calculate total minimum cost
    let mut total_cost = 0;

    for i in 1..n {
        let dx = (x_coords[i] - x_coords[i - 1]).abs();
        let dy = (y_coords[i] - y_coords[i - 1]).abs();

        // Calculate movement cost
        let cost = if x_coords[i] < a || x_coords[i] > b || y_coords[i] < c || y_coords[i] > d {
            dx + dy + 1 // Additional cost if outside boundaries
        } else {
            dx + dy // Normal cost
        };

        total_cost += cost;
    }

    // Output the total minimum cost
    println!("{}", total_cost);
}
