use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for N, A, B, C, D
    let first_line = lines.next().unwrap_or_else(|_| panic!("Failed to read input"));
    let first_line = first_line.unwrap_or_else(|_| panic!("Failed to unwrap input line"));
    let mut params = first_line.split_whitespace();
    
    let n: usize = params.next().unwrap().parse().unwrap();
    let a: i32 = params.next().unwrap().parse().unwrap();
    let b: i32 = params.next().unwrap().parse().unwrap();
    let c: i32 = params.next().unwrap().parse().unwrap();
    let d: i32 = params.next().unwrap().parse().unwrap();

    // Check for valid N
    if n == 0 {
        println!("1"); // Return 1 and terminate if N is 0
        return;
    }

    // Read the coordinates
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    
    for _ in 0..n {
        let line = lines.next().unwrap_or_else(|_| panic!("Failed to read input"));
        let line = line.unwrap_or_else(|_| panic!("Failed to unwrap input line"));
        let mut coords = line.split_whitespace();
        let x_coord: i32 = coords.next().unwrap().parse().unwrap();
        let y_coord: i32 = coords.next().unwrap().parse().unwrap();
        x.push(x_coord);
        y.push(y_coord);
    }

    // Calculate the total minimum cost
    let mut total_cost = 0;

    for i in 1..n {
        let dx = x[i] - x[i - 1];
        let dy = y[i] - y[i - 1];

        // Calculate the cost based on boundary constraints
        let cost = if x[i] < a || x[i] > b || y[i] < c || y[i] > d {
            (dx.abs() + dy.abs()) + 1 // Additional cost if outside boundaries
        } else {
            dx.abs() + dy.abs() // Normal cost
        };

        total_cost += cost;
    }

    // Output the total minimum cost
    println!("{}", total_cost);
}
