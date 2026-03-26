use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: usize = input.trim().parse().expect("Invalid input for number of vertices");
    
    // Ensure n is greater than 0 as per the specification
    if n == 0 {
        eprintln!("Number of vertices must be greater than 0.");
        return;
    }

    let mut p = vec![(0, 0); n]; // Vector to hold the coordinates

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate input"))
            .collect();
        
        // Ensure we have exactly two coordinates
        if coords.len() != 2 {
            eprintln!("Each vertex must have exactly two coordinates.");
            return;
        }
        
        p[i] = (coords[0], coords[1]);
    }

    let mut menseki = 0.0;

    for i in 0..n {
        let next_index = (i + 1) % n;
        menseki += (p[i].0 - p[next_index].0) as f64 * (p[i].1 + p[next_index].1) as f64;
    }

    println!("{:.1}", menseki / 2.0);
}
