use std::io::{self, BufRead};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of circle pairs
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the circle parameters
        let line = lines.next().unwrap().unwrap();
        let params: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let a = Circle {
            x: params[0],
            y: params[1],
            r: params[2],
        };
        let b = Circle {
            x: params[3],
            y: params[4],
            r: params[5],
        };

        // Calculate the squared distances and radii
        let distance_sq = (a.x - b.x).powi(2) + (a.y - b.y).powi(2);
        let radii_sum_sq = (a.r + b.r).powi(2);
        let radii_diff_sq = (a.r - b.r).powi(2);

        // Determine the relationship between the circles
        if distance_sq > radii_sum_sq {
            println!("0");
        } else if distance_sq < radii_sum_sq && distance_sq > radii_diff_sq {
            println!("1");
        } else if distance_sq < radii_diff_sq {
            if a.r > b.r {
                println!("2");
            } else {
                println!("-2");
            }
        } else {
            println!("1");
        }
    }
}
