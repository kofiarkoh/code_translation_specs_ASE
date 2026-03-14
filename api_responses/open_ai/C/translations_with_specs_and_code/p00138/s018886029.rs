use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Run {
    p: i32,
    t: f64,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut g: Vec<Vec<Run>> = vec![vec![]; 3];

    // Read input
    for i in 0..3 {
        for j in 0..8 {
            let mut input = String::new();
            handle.read_line(&mut input).expect("Failed to read line");
            let parts: Vec<&str> = input.split_whitespace().collect();
            let p: i32 = parts[0].parse().expect("Failed to parse integer");
            let t: f64 = parts[1].parse().expect("Failed to parse double");
            g[i].push(Run { p, t });
        }
    }

    // Sort each set based on the double values
    for i in 0..3 {
        g[i].sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    // Output the first two pairs from each sorted set
    for i in 0..3 {
        for j in 0..2 {
            println!("{} {:.2}", g[i][j].p, g[i][j].t);
        }
    }

    // Find the two smallest double values across all sets
    let mut min1 = Run { p: 0, t: f64::INFINITY };
    let mut min2 = Run { p: 0, t: f64::INFINITY };

    for i in 0..3 {
        for j in 2..8 {
            if g[i][j].t < min1.t {
                min2 = min1.clone();
                min1 = g[i][j].clone();
            }
        }
    }

    // Output the two smallest pairs
    println!("{} {:.2}", min1.p, min1.t);
    println!("{} {:.2}", min2.p, min2.t);
}
