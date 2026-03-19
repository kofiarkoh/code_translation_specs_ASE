use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut p = vec![(0, 0); 100]; // Allocate space for 100 points, but we will only use n

    // Read the points
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        p[i] = (coords[0], coords[1]);
    }

    let mut menseki = 0.0;

    // Calculate the area using the shoelace formula
    for i in 0..n {
        let next_index = (i + 1) % n; // Wrap around using modulo
        menseki += (p[i].0 - p[next_index].0) as f64 * (p[i].1 + p[next_index].1) as f64;
    }

    // Print the result formatted to one decimal place
    println!("{:.1}", menseki / 2.0);
}
