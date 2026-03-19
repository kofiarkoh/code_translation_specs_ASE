use std::io::{self, Read};

const EPS: f64 = 1e-5;

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        let q: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // Spec: non-integer data behavior not inferable; stop quietly.
        };

        if q == -1 {
            break; // termination value
        }

        // Spec precondition: q > 0 for non-terminating inputs.
        // If violated, behavior is not reliably defined; we mirror C's lack of handling.
        let qf = q as f64;

        let mut px: f64 = qf / 2.0;
        let mut x: f64;

        loop {
            // Newton-Raphson for solving x^3 = q:
            // x_{n+1} = x_n - (x_n^3 - q) / (3*x_n^2)
            x = px - (px * px * px - qf) / (3.0 * px * px);
            px = x;

            if (x * x * x - qf).abs() < EPS * qf {
                break;
            }
        }

        println!("{:.6}", x);
    }
}
