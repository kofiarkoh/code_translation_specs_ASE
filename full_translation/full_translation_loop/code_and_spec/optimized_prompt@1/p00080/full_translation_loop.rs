use std::io::{self, BufRead};

const EPS: f64 = 1e-5;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(q) = line.trim().parse::<i32>() {
            if q == -1 {
                break; // Termination signal
            }
            let cube_root = compute_cube_root(q as f64);
            println!("{:.6}", cube_root);
        }
    }
}

fn compute_cube_root(q: f64) -> f64 {
    let mut px = q / 2.0; // Initial guess
    let mut x;

    loop {
        x = px - (px * px * px - q) / (3.0 * px * px);
        if (x * x * x - q).abs() < EPS * q {
            break;
        }
        px = x;
    }

    x
}
