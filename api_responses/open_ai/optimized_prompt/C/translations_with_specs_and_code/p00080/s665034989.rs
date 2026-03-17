use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.expect("Failed to read line");
        let q: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        if q == -1 {
            break; // Termination signal
        }

        if q < 0 {
            continue; // Skip negative inputs, as per specification
        }

        let mut px = q as f64 / 2.0;
        let eps = 1e-5;

        loop {
            let x = px - (px * px * px - q as f64) / (3.0 * px * px);
            if (x * x * x - q as f64).abs() < eps * q as f64 {
                println!("{:.6}", x);
                break;
            }
            px = x;
        }
    }
}
