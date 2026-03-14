use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Run {
    p: i32,
    t: f64,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut g: [[Run; 8]; 3] = Default::default();
    
    // Read input
    for i in 0..3 {
        for j in 0..8 {
            let mut input = String::new();
            handle.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            g[i][j].p = parts[0].parse().unwrap();
            g[i][j].t = parts[1].parse().unwrap();
        }
    }

    // Sort the array based on the 't' field
    for i in 0..3 {
        for j in 0..7 {
            for k in (j + 1..8).rev() {
                if g[i][k - 1].t > g[i][k].t {
                    g[i].swap(k - 1, k);
                }
            }
        }
    }

    // Print the first two elements of each row
    for i in 0..3 {
        for j in 0..2 {
            println!("{} {:.2}", g[i][j].p, g[i][j].t);
        }
    }

    // Find the two minimum 't' values
    let mut min1 = Run { p: 0, t: 100.0 };
    let mut min2 = Run { p: 0, t: 100.0 };

    for i in 0..3 {
        for j in 2..8 {
            if g[i][j].t < min1.t {
                min2 = min1.clone();
                min1 = g[i][j].clone();
            }
        }
    }

    // Print the minimum values found
    println!("{} {:.2}", min1.p, min1.t);
    println!("{} {:.2}", min2.p, min2.t);
}
