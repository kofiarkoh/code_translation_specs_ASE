use std::io::{self, Read};

const EPS: f64 = 1e-5;

fn main() {
    // Read entire stdin, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    for tok in input.split_whitespace() {
        let q: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // Unspecified beyond input failure; stop processing.
        };

        if q == -1 {
            break; // Sentinel: terminate without output.
        }

        // Spec: intended for positive q; behavior for q <= 0 is unspecified.
        let qf = q as f64;

        // p0 = q/2 as double
        let mut px = qf / 2.0;
        let x: f64;

        // Do-while loop: must execute at least once, and print the final x
        // produced when the loop terminates.
        loop {
            x = px - (px * px * px - qf) / (3.0 * px * px);
            px = x;

            if (x * x * x - qf).abs() < EPS * qf {
                break;
            }
        }

        // Exactly one line per processed integer, with exactly six digits after decimal.
        println!("{:.6}", x);
    }
}
